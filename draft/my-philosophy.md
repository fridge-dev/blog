This is a running post containing short snippets of my personal philosophy, habits, and best practices. Some of these may be turned into larger posts later, but for the most part these don't merit a single page post.

A lot of these are written to my past self as the target reader. Good luck!

## Don't over-work

Cal Newport: Work at a natural pace. Imagine you keep working at your current pace for the rest of your career. Is it sustainable? Rushing to reach a milestone or deadline is often a trap. And it can be a self induced trap! Not always evil managers. <link to other post about telling your managers when you’re too busy>. Be careful for that trap. The start of the next project or milestone is right around the corner and sometimes starts even before the previous milestone properly wraps up (yay tech debt). Choose a pace to work at that you truly believe is sustainable and energizing for work hours and for home life. Be strict and disciplined. My rule of thumb is there should MAYBE be one period per year (of 2-6 weeks let’s say) where you are rushing to cross the finish line. Maybe once per year. That means either 0 or 1 times per year. If your current project or milestone feels important, imagine if you are somehow strictly limited to overworking for one deadline per year. You have a token bucket of capacity 1 and refill rate of 1 per year. If you use your precious 1 token on this milestone, will you regret not having it available if something truly important comes up later in the year?

Side note: are you a founder and have a external existential pressure demanding constant overwork? This post isn’t for you. You might very well need to be constantly working as your life. 

Do you love working extra hours? As long as your working pace is on the healthy side of the draining<->energizing spectrum. And as long as you’re not a team lead or manager who has a significant impact on your team or org’s working culture. Lead by example! ICs will implicitly understand what's expected of them based on how their leaders behave.

## Test philosophy

Imagine a future dev is allowed to make any (reasonably well-intended) changes to the source so long as the tests pass. Any changes where the tests pass, that innocent dev is allowed to deploy those changes to production. In this case, what tests would you write?

_"Reasonably well-intended" means you don't have to account for someone adding a branch like `if arg == 50821358 { panic!() }`._

## Favor understandability over ergonomics and DRY

Moved to full post, see [2025-01-16-understandability-over-dry.md] TODO make links work

## Premature abstractions, KISS, YAGNI, and refactoring

I’ve worked on systems that felt like they were in a deadlock to change because of unnecessary abstractions and making things generic. I’ve worked on systems that were relatively easier to change since they solved only the immediate problem and, although documentation shows the authors were aware of potential ideas for extension/new consumers, preparing for those was not the main objective of the code. This mentality is complimentary with refactoring.

You’ve gotta be okay with refactoring, and you should start to budget every project to have some amount of **pre-req refactoring** to make the solution's end-state more cohesive in the bigger picture of the application's architecture. I call this "prefactoring". The refactor might only make sense in the context of this project’s implementation, but since this project being implemented is a certainty (vs the projects that are a hypothetical future idea), then it makes sense. But we should prefer refactor that independently makes sense. Main point: there will always be too many things to refactor. Refactor what makes sense for the current project, not potential future ones.

We have a tendency to optimize for the allusion of the “free win” type of feature-add. In practice, we rarely get all of the subtle requirements and impl details correct that we can add a feature with nearly no impl. So why spend mental energy and impl abstractions to try to achieve this ahead of time? It’s because the allure of it is so satisfying. _“This new feature loosely is similar enough to an existing functionality, so we can add that feature in less than a day without having to add additional tests.”_ Boy oh boy would that make the product team think highly of the engineering decisions we’ve made in the past! Everyone would think we were so clever to foresee this coming and lay down a golden path to make this so easy. No. It basically never happens like that. Stop making it harder to understand how the system functions today with **potential**, hypothetical, maybe-could-be-funded-some-day-so-we-should-account-for-it ideas of tomorrow. My rule of thumb is if it's not definitely funded on the roadmap for the next 3 months, then don't code for it. It may never happen. Even if you're pretty sure it will happen by the end of the year because it's important for another team's goal. Things change! Until you're actually building that feature, don't build anything for that feature.

Shorter version: Refactoring is okay, and we shouldn't prematurely optimize to strive for 0 refactoring in the future. Programmers always want the satisfaction of being able to tell a product team "oh this feature you're asking for will only take 1 day because of how clever my abstraction were in the past". This basically never happens! Don't optimize for that scenario! Be more humble, assume any attempt to abstract for potential future ideas will be wrong, and also know that abstractions for unrealized features are a conceptual debt that will be a burden for the team to understand and maintain.

## When/how to take tech debt

As an engineer, don’t take unintentional, implicit tech debt. Start by taking no shortcuts and going straight for the ideal end state. If the business context requires shortcuts or suboptimal trade off, be explicit about it in design docs or code comments. If you accept implicit tech debt, new engineers will learn that this is the normal quality of software and not understand why we're taking a shortcut, or may not even recognize that we're taking a shortcut at all.

## Limit your fallback branches to avoid operational modes

Token buckets and rate limiting fallback logic/operational mode. Deal with the devil: steady state availability vs LSE overload. When writing an exceptional branch that has a non-trivial cost, think about what if that starts to happen on every single request? Example: retries. Example: stacktrace logging (link hotspot removal of stacktrace). Example: cache miss/population.

## Communication: Don't blindly restate terms you don't understand

If you don’t precisely understand what a term means, don’t use it. This happens a lot with buzzwords, but can happen with any technical terms. One person's mental model of a term can emphasize different details than others' understanding. If you think others might not have the same understanding as you, use a literal definition of the term instead. If it's your first time discussing a topic with a group, and well-defined terms are not established, err on the side of over specifying. It might sound silly, but it will remove ambiguity and avoid wasting time on miscommunication.

Examples: replace "linter" with "automated static detection of common bugs at build time".

Useful for alignment: detect and predict when ideas or topics are prone to different understandings. Do we keep having conversations about the same ideas but have frequent clarifying questions? Identify recurring confusing idea, introduce terms with obvious definition, and lead by example and use those terms. Don't mindlessly repeat terms that others say if you don't understand the term.

## Communication: Pay attention to mis-use of solution and generic abstraction

Conversationally, I've noticed people will frequently talk about a specific solution of technology when the generic abstraction is more appropriate. You don't have to correct someone if they're being overly specific, but if you notice it, it could be a sign that we're not thinking through a problem abstractly enough.

Example: dynamic config vs static config, 

## Communication: History is important

When explaining a system, it's helpful to explain the history and evolution of the architecture and the decisions and the team's thinking at the time, rather than just explain the current state. Understanding how it evolved and decisions made along the way helps new members build intuition about what's important for the system and business and how things may continue to evolve.

## How to do estimation

A useful exercise is to force yourself to write 2 example things that are out of scope. If you only write what's in scope, you won't know where to draw the line of what’s in scope and out of scope. Doing this exercise will help you remember things that are in scope and should be accounted for in an estimation.

Examples to consider: Does your scope/estimation/timeline account for:
* Troubleshooting a new integration of a technology (be pessimistic, new integrations rarely work right on the first try)
* Writing integration tests
* Manual testing in a devo environment
* Debugging devo/prod environment
* Performing a security review
* Any manual deployment or audit of a production environment
* Creating automated alarming
* Writing operational documentation
* Cleaning up tech debt / feature flags
* Rolling back a bug and deploying a new fix
* Your holiday schedule (only relevant for ECD, not effort estimation)
* Your other work obligations (only relevant for ECD, not effort estimation)

Even if a project _eventually_ has to include all of these things, it's okay for the scope that you're estimating to explicitly exclude some of these. As long as you explicitly exclude them! 

## Design: Sequence diagramming is OP

Sequence diagram is super underutilized. Use it to efficiently communicate ideas to coworkers. Use it to prove to yourself you fully understand your solution. Often, sequencing of steps are the details where the devil is hiding, and we'd rather learn those tricky areas during design time when it's cheaper to change things rather than during implementation time.

Get familiar with a sequence diagramming tool so that it's low friction and you enjoy using it (I personally love plant UML). You will reach for it more often. Writing with pen and paper taking a picture of it is waaay better than nothing!

## Design: Try to break the design

As an engineer, you need to train your brain to eagerly search the state space of a design and find the first possible way to break it. Treat a design review like you're a TLA+ evaluator trying to find an inconsistency. Same applies for drafting your own designs.

good leader: find the next point of failure (in a design) as soon as possible. Dive straight into the details (quantified if possible) of what it would take to solve something. Is it ridiciuluous and crazy? Is only one specific part of it ridiculuous and crazy, and the rest is reasonable? You have just broken down the problem into a more specific, more distilled problem.

## Design: Design docs are a tool to save you time

Moved to dedicated post.
