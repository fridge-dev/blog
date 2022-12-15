<where to put this section?> I primarily use rust and go on a day-to-day basis, and I have used java and python heavily in the past too. I think java is the most widely used, and potentially the community of which new developers may benefit the most, so I've targeted this post in java. I may make a dynamic toggle allowing reader to flip parts of the post to be for other langauges too, if I get time for it.

<introduce article: Dependency Wiring Pattern. Target audience is for top-level application developers. Also applies to Library developers, but I see it wrong in top-level applications much more often than Libraries.>

<where to put this section?> what is a dependency? A class has fields or "members" (in java). ~~Those fields are either a raw data structure (e.g. HashMap) or a dependency.~~ Data structures/primitive types **can be** DI'd if it makes sense. Behavioral dependencies should **always** be DI'd.

Okay, let's establish one thing. Dependency injection (DI) is good! We like DI. DI helps us keep code decoupled (as we'll see) and almost more importantly, DI helps us keep our unit tests maintainable. I've had colleagues in the past argue that we shouldn't augment our code just for UT code quality, to which I would respond that test code should be considered a first class citizen of your projection, and it is absolutely worth it ([to some extent](Article idea: when testability can go too far; focusing too much on DRY in tests can hurt flow of logic; reading a test case source code should be like reading an instruction manual/game rule book, hopefully you have to flip to least amount of other sections to understand the rule)).

What *is* DI? My concrete definition for "pure DI" is when your `class` has a constructor that takes every dependency as an arg, and there's nothing[1] in the constructor besides assigning the instance's fields.

[1] The one exception I'm okay with for languages that can have null/nil values is checking for null/nil and returning an error.

Let's see DI in action. I will use a toy example where I am writing a library that offers Bicycle functionality, and exports a single class, `Bicycle`, for the consumer to use.

https://en.wikipedia.org/wiki/List_of_bicycle_parts#/media/File:Bicycle_diagram-en.svg

```java
// === Our library code ===

public class Bicycle {
    private final Wheel backWheel;
    private final Wheel frontWheel;
    private final Frame frame;
    private final Seat seat;
    private final Handles handles;

    public Bicycle(
            final Wheel backWheel,
            final Wheel frontWheel,
            final Frame frame,
            final Seat seat,
            final Handles handles
    ) {
        this.backWheel = backWheel;
        this.frontWheel = frontWheel;
        this.frame = frame;
        this.seat = seat;
        this.handles = handles;
    }

    // ...
}
```

I think you should pretty much always use pure DI for every class. It makes things so much simpler to test, it cleanly separates how your object is constructed from how it behaves, and it removes any mental energy spent deciding how to allow the creation of your object. The problem with doing pure DI for every class is eventually, some caller at the top level will need to construct your `Bicycle` class, and that means they have to construct `Bicycle`'s dependencies first, and since we're doing pure DI everywhere, they will also have to construct the dependencies of your dependencies, and dependencies of your dependencies of your dependendencies, etc. All of a sudden, our simple `Bicycle` API with good abstraction and encapsulation requires consumers to have a deep knowledge of our internal dependency structure. It also requires all of our classes to be public. Both can result in coupling and maintainability issues.

## Problem

```java
// === Our library code ===

public class Wheel {
    private final Rim rim;
    private final Tire tire;
    private final Spokes spokes;

    public Wheel(
        final Rim rim,
        final Tire tire,
        final Spokes spokes
    ) {
        this.rim = rim;
        this.tire = tire;
        this.spokes = spokes;
    }

    // ...
}

public class Frame { /* ... */ }
public class Seat { /* ... */ }
public class Handles { /* ... */ }

// === Consumer code ===

public class MyApplication {

    public static void main() {
        // Low level primitives that exist externally of our library.
        RubberSupplier rubberSupplier = Environment.load(/* ... */);
        ThickMetal rawMetalFrame = Environment.load(/* ... */);
        Wrench wrench = Environment.load(/* ... */);
        List<Bolt> bolts = Environment.load(/* ... */);
        // ...

        // Wire all Bicycle internal dependencies together. (What a mess!)
        Wheel backWheel = new Wheel(
                new Rim(/* ... */),
                new Tire(/* ... */),
                new Spokes(/* ... */)
        );
        Wheel frontWheel = new Wheel(
                new Rim(/* ... */),
                new Tire(/* ... */),
                new Spokes(/* ... */)
        );

        Frame frame = new Frame(/* ... */);
        Seat seat = new Seat(/* ... */);
        Handles handles = new Handles(/* ... */);

        Bicycle bicycle = new Bicycle(
                backWheel,
                frontWheel,
                frame,
                seat,
                handles
        );

        // ...
    }
}
```

How can we make this better for the caller?

## Solution A - Move construction into Bicycle

Ideally, the caller should only provide the dependencies that are external to our library, meaning classes/interfaces that are defined not in our code.

```java
// === Our library code ===

public class Bicycle {
    private final Wheel backWheel;
    private final Wheel frontWheel;
    private final Frame frame;
    private final Seat seat;
    private final Handles handles;

    public Bicycle(
            final RubberSupplier rubberSupplier,
            final ThickMetal rawMetalFrame,
            final Wrench wrench,
            final List<Bolt> bolts,
            // ...
    ) {

        Wheel backWheel = new Wheel(
                new Rim(/* ... */),
                new Tire(/* ... */),
                new Spokes(/* ... */)
        );
        Wheel frontWheel = new Wheel(
                new Rim(/* ... */),
                new Tire(/* ... */),
                new Spokes(/* ... */)
        );

        Frame frame = new Frame(/* ... */);
        Seat seat = new Seat(/* ... */);
        Handles handles = new Handles(/* ... */);

        // Finally construct the object
        this.backWheel = backWheel;
        this.frontWheel = frontWheel;
        this.frame = frame;
        this.seat = seat;
        this.handles = handles;
    }
}

// === Consumer code ===

public class MyApplication {

    public static void main() {
        // Low level primitives that exist externally of our library.
        RubberSupplier rubberSupplier = Environment.load(/* ... */);
        ThickMetal rawMetalFrame = Environment.load(/* ... */);
        Wrench wrench = Environment.load(/* ... */);
        List<Bolt> bolts = Environment.load(/* ... */);
        // ...

        Bicycle bicycle = new Bicycle(
                rubberSupplier,
                rawMetalFrame,
                wrench,
                bolts,
                // ...
        );

        // ...
    }
}
```

Much better! Now our consumer has a simple, minimal constructor for them to deal with. But now unit testing our Bicycle class is basically impossible. How can we get the benefits of pure DI on the bicycle class without exposing a complex constructor to our consumer?

## Solution B - Move construction into Wiring class.

I introduce what I call the "Wiring" class: It is a class whos *behavior* is to wire all of the dependencies together and construct the top level object. The name/terminology isn't super important. If your team wants to use a different name, then do it. Just be consistent! I like the name because it's a distinct name (no existing pattern with that name) and it's unambiguous with other creational patterns (Factory, Builder, etc).

```java
// === Our library code ===

public class Bicycle {
    private final Wheel backWheel;
    private final Wheel frontWheel;
    private final Frame frame;
    private final Seat seat;
    private final Handles handles;

    public Bicycle(
            final Wheel backWheel,
            final Wheel frontWheel,
            final Frame frame,
            final Seat seat,
            final Handles handles
    ) {
        this.backWheel = backWheel;
        this.frontWheel = frontWheel;
        this.frame = frame;
        this.seat = seat;
        this.handles = handles;
    }

    // ...
}

public class BicycleWiring {
    public BicycleWiring() {}

    public Bicycle configure(
            final RubberSupplier rubberSupplier,
            final ThickMetal rawMetalFrame,
            final Wrench wrench,
            final List<Bolt> bolts,
            // ...
    ) {
        // Wire all internal dependencies together.
        Wheel backWheel = new Wheel(
                new Rim(/* ... */),
                new Tire(/* ... */),
                new Spokes(/* ... */)
        );
        Wheel frontWheel = new Wheel(
                new Rim(/* ... */),
                new Tire(/* ... */),
                new Spokes(/* ... */)
        );

        Frame frame = new Frame(/* ... */);
        Seat seat = new Seat(/* ... */);
        Handles handles = new Handles(/* ... */);

        // Finally construct the object
        return new Bicycle(
                backWheel,
                frontWheel,
                frame,
                seat,
                handles
        );
    }
}

// === Consumer code ===

public class MyApplication {

    public static void main() {
        // Low level primitives that exist externally of our library.
        RubberSupplier rubberSupplier = Environment.load(/* ... */);
        ThickMetal rawMetalFrame = Environment.load(/* ... */);
        Wrench wrench = Environment.load(/* ... */);
        List<Bolt> bolts = Environment.load(/* ... */);
        // ...

        Bicycle bicycle = new BicycleWiring().configure(
                rubberSupplier,
                rawMetalFrame,
                wrench,
                bolts,
                // ...
        );

        // ...
    }
}
```

Voila! Now everyone is happy. Our library's consumers have a simple, minimal API that isn't concerned with library's internal dependency structure. We (library maintainers) can freely refactor our internal dependency structure and easily add/update UTs.

If your library is exporting more than a single type, then you can make the Wiring class contain common/re-usable external dependencies and define different `configureX()`, `configureY()` methods. If your library has a single type that it exports, then congratulations on producing a nice layer of abstraction! You can feel free to make the Wiring class's only method a static method. Your consumers will love you!

You might think that all of that dependency wiring looks complicated, but to that I say 2 things:

1. All of the dependency wiring complexity is contained within a single `BicycleWiring` class whose single responsibility is to wire the dependencies! The code must live somewhere. At least we contain it in a single location.
1. If the wiring truly is many hundreds of lines of code with many private methods, and it's difficult to build a mental model of your internal dependency structure, then this is a code smell that your internal dependency structure is not well abstracted.

<somewhere> some would say it's nicer to have a DI FW to do all of that for them, to that I say that [I don't think DI frameworks are needed](link to blog post explaining manual DI is not that hard)

Let's look at how we can recursively apply the wiring pattern to fix the code smell mentioned in #2 above.

## Wiring individual modules/packages

### Creating sub-modules

First, let me establish some terminology:

Language|**Library** - Single project/repository|**Module** - Logical sub-units for organizing code within a single project, usually a single directory|**Object** Definition of a type, usually one or a few in a single file
rust|crate|mod(ule)|struct/enum/trait
java|library|package|class/enum/interface
golang|module|package|struct/interface

Once a library's source code becomes large enough, it will gain benefit from organizing code into modules, where each module is responsible for 1 logical unit's amount of conceptual complexity. And modules can be nested. So a single module might contain many low level sub-modules (for abstracting complexity) and combine them into a single "medium level" concept that is exposed to the higher level library that uses it to fulfill the public API contract.

Be aware that you may have a sub-conscious desire to make your module/sub-module tree roughly "balanced". There's no reason it has to be, it's just a sub-conscious desire for how to (falsely!) organize something. In reality, you will probably have one sub-module that has many layers of intermediate sub-modules, while most of your modules will just be 1 layer deep. That is normal. Create new sub-modules when you feel it makes sense to for that current module, NOT because you want to match the depth of sub-modules elsewhere in your library.

### Wiring sub-modules

This entire post was written for how to make the Library<->Consumer boundary as clean as possible, without sacrificing maintainability of Library. We can simply apply all of the same arguments to the Module<->Library boundary to get the same effect, but nested. This requires somewhat of a mindshift, because this means (usually) the same team/organization will be maintaing both the primary code (sub-module) and consumer (library/super-module). Often times one of the things that slows down organizations most is internal code complexity that's behind a nicer API. If we can hold our interior abstractions to the same quality bar as our external facing APIs, then we will be helping our future selves.

Remember:

1. Use pure DI
1. Define a Wiring class that only uses external-to-module types as dependencies/args

```java
// === Wheel module code ===

public class Wheel {
    private final Rim rim;
    private final Tire tire;
    private final Spokes spokes;

    public Wheel(
        final Rim rim,
        final Tire tire,
        final Spokes spokes
    ) {
        this.rim = rim;
        this.tire = tire;
        this.spokes = spokes;
    }

    // ...
}

public class WheelWiring {
    public static Wheel configure(
        final RubberSupplier rubberSupplier,
        final MetalRing metalRing,
        final TreadStyle treadStyle,
        // ...
    ) {
        RubberRingFactory rubberRingFactory = new RubberRingFactory(rubberSupplier, /* ... */);

        return new Wheel(
                new Rim(metalRing, /* ... */),
                new Tire(rubberRingFactory, treadStyle, /* ... */),
                new Spokes(/* ... */)
        );
    }
}
```

```java
// === Top level library code ===

public class Bicycle {
    private final Wheel backWheel;
    private final Wheel frontWheel;
    private final Frame frame;
    private final Seat seat;
    private final Handles handles;

    public Bicycle(
            final Wheel backWheel,
            final Wheel frontWheel,
            final Frame frame,
            final Seat seat,
            final Handles handles
    ) {
        this.backWheel = backWheel;
        this.frontWheel = frontWheel;
        this.frame = frame;
        this.seat = seat;
        this.handles = handles;
    }

    // ...
}

public class BicycleWiring {
    public BicycleWiring() {}

    public Bicycle configure(
            final RubberSupplier rubberSupplier,
            final ThickMetal rawMetalFrame,
            final Wrench wrench,
            final List<Bolt> bolts,
            // ...
    ) {
        // Wire all internal dependencies together.
        Wheel backWheel = WheelWiring.configure(rubberSupplier, /* ... */);
        Wheel frontWheel = WheelWiring.configure(rubberSupplier, /* ... */);

        Frame frame = new Frame(/* ... */);
        Seat seat = new Seat(/* ... */);
        Handles handles = new Handles(/* ... */);

        // Finally construct the object
        return new Bicycle(
                backWheel,
                frontWheel,
                frame,
                seat,
                handles
        );
    }
}
```

== Other benefits ==

=== Functional integ tests ===

- applies to both library and modules

=== Export interface, not class ===

- Wiring.confiugre() should return an interface
