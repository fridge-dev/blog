Plan to start with something subpar and then improve it. If you hold yourself to a bar of near perfection, you’ll take super long to start. And when you start with something, that’s when you begin learning at such a high rate. Example: piano. Example: learning how to do a DIY home repair. Expect the first one to be messy and go poorly. Give yourself permission to do sloppy, “just good enough” job. Maybe you’re not proud of it, but hey it solved the immediate problem and now you learned from it and learned about the higher level framework of problem solving.

Alarm on dynamic override of static config

Premature abstractions. I really wish I had an example of removing one. I’ve worked on systems that felt like they were in a deadlock to change because of unnecessary abstractions and making things generic. I’ve worked on systems that were relatively easier to change since they solved only the immediate problem and, although documentation shows the authors were aware of potential ideas for extension/new consumers, preparing for those was not the main objective of the code. This brings me to another point. You’ve gotta be okay with refactoring, and you should start to budget every project to have some amount of pre-req refactoring to make solving the intended problem make more cohesive sense in the bigger picture. I call this prefactoring. The refactor might only make sense in the context of this project’s implementation, but since this project being implemented (vs not being implemented) is a certainty, an inevitability, then it makes sense. But we should prefer refactor that independently makes sense. Main point: there will always be too many things to refactor. Refactor what makes sense for the current project, not potential future ones. We have a tendency to optimize for the allusion of the “free win” type of feature-add, or project. In practice, we rarely get all of the subtle impl details and minute requirements correct that the feature is a near no-op impl. So why spend mental energy and impl abstractions to try to achieve this? It’s because the allure of it is very satisfying. “This feature loosely resembles this existing functionality and shares some core elements of another, so we can implement this in less than a day without having to add additional tests.” Boy oh boy would that make the product team think highly of the engineering decisions we’ve made in the past! Everyone would think we were so clever to foresee this coming and lay down a golden path to make this so easy. No. It basically never happens like that. Stop making it harder to understand how the system functions today with **potential**, hypothetical, might-could-be-funded-some-day-so-we-should-account-for-it ideas of tomorrow. My rule of thumb is if it's not definitely (90% confidence) in the next 6 months of roadmap, don't code for it. It may never happen. TODO incorporate example of loading the dishwasher? lol

Token buckets and rate limiting fallback logic/operational mode. Deal with the devil: steady state availability vs LSE overload. When writing an exceptional branch that has a non-trivial cost, think about what if that starts to happen on every single request? Example: retries. Example: stacktrace logging (link hotspot removal of stacktrace). Example: cache miss/population.

Compare cyberspace sci-fi to viewing metrics in a dashboard. It helps you build a mental image of what happens and at what frequency. Drawing up those rough mental models into a diagram that others can see can sometimes be a breakthrough in others’ learnings. It will help the team be more productive. Even if you think the textual explanation is understandable and making a diagram will take significant effort, it can still be impactful to do it. Lower your bar (link other blog post) and start by drawing a pen and paper diagram just so you can show yourself. Avoid the fuss of software tools and the line not connecting in the right curve between two boxes or where to drag/move the text each time you add a new box or line to the diagram.

How to write 5 whys + AIs + LL -> use a bubble diagram on pen and paper.

Common distr system tool: external logical clock (if you can enforce strong consistency). Box out(?)

Common distr system tool: WAL and repl FSM

To be a good mentee, you need to be VULNERABLE. You can only reach that level of comfort if you have TRUST with your mentor. If you don’t trust your mentor, consider finding a new mentor. … to be a good mentor, you need to be trustworthy, honest, and warm. And lead by example; be vulnerable. If you show examples of stuff you’re going through and your private POV, mentee will feel more comfortable to open up. How should a mentor practice this? I’m not really sure. It comes very naturally to me (this is how I would talk to most people in life), so I never really had to think about practicing it. I would say just go talk to more people and really listen to them and understand them. It’s prob a whole university level course on learning how to be empathetic.
How to have a useful 1:1 w a mentor?:
- Explain random situations your past week.
- Explain how you thought about it. What you did/didn't, why.


Cal Newport: Work at a natural pace. Imagine you keep working at your current pace for the rest of your career. Is it sustainable? Rushing to reach a milestone or deadline is often a trap. And it can be a self induced trap! Not always evil managers. <link to other post about telling your managers when you’re too busy>. Be careful to fall for that trap. The start of the next project or milestone is right around the corner and sometimes starts even before the previous milestone properly wraps up (yay tech debt). Choose a pace to work at that you truly believe is sustainable and energizing for work hours and for home life. Be strict and disciplined. My rule of thumb is there should MAYBE be one period per year (of 2-6 weeks let’s say) where you are rushing to cross the finish line. Maybe once per year. That means either 0 or 1 times per year. If your current project or milestone feels important, imagine hypothetically that you are somehow strictly limited to overworking for one milestone per year. You have a token bucket of capacity 1 and refill rate of 1 per year. If you use your precious 1 token on this milestone, will you regret not having it available if something truly important comes up later in the year? Side note: are you a founder and have a legitimate external existential pressure demanding constant overwork? This post isn’t for you. You might very well need to be constantly working as your life. Do you love working extra hours? As long as your working pace is on the healthy side of the draining<->energizing spectrum. * and as long as you’re not a team lead or mgr who has a significant impact on your team or org’s working/social culture. Lead by example!

You can sometimes tell how a backend architecture evolved just based on the UX flows. Example: Google docs UX models the viewers/editors as members of the doc with a role selection. Spotify playlist models sharing to RO followers as a separate dropdown menu option and flow than sharing to collaborators. Is there a phenomenon for when backend abstractions leak into the UX flow like this? Is it just a subconscious coupling of the UX designers’ mental models with the discussion of engineering internals? Is it a sign that Eng and UX are well connected to each other, and/or a small group of people who wear more hats on avg.

ABP: if you don’t understand what a buzzword means, don’t use it. If you think others don’t know what it means or don’t have the same understanding as you, use a literal definition of what you’re trying to say to remove ambiguity, even if it sounds silly/redundant. Same with shorthand. Over specify! Examples: TODO linter- automated static detection of common bugs at build time.

Useful for alignment: detect and predict when terms or topics are prone to different understandings. Pre empt confusion by clarifying things before wasting conversational time. Find confusing ideas, introduce terms with obvious definition, and lead by example and use those terms. Don't mindlessly repeat terms that others say (see ABP buzzwords).

Estimation: useful exercise to write 2 example things that are out of scope. It will help you place the line of what’s in scope and out of scope.

ABP: When explaining a system, it's helpful to explain the history and evolution of the architecture and the decisions and the thinking at the time, rather than just explain the current state. Understanding how it evolved and decisions made along the way helps build intuition about what's important for the system and how it may continue to evolve.

ABP: UML modeling for data repre is good. For behavior classes is really hard to get right on first try.
    - not a BP, just a thought?

ABP: Sequence diagram is super underutilized. Get familiar with a seq diagraming tool so that you enjoy using it. I love plant UML. Use it to efficiently communicate ideas to coworkers. Use it to prove to yourself you fully understand your solution.

ABP: Pay attention to the "soft" in software. It means it's malleable. We can change it. Refactoring is okay and we shouldn't prematurely optimize to strive for 0 refactoring in the future. If you're programming like how a hardware engineer would build hardware, or civil/mechanical, then you're doing it wrong. - wtf did I mean?

ABP: As an engineer, you need to train your brain to eagerly search the state space of a design and find the first possible way to break it. Kind of like TLA+.

ABP(Elsa): don’t take unintentional debt. Start by taking no shortcuts and going straight for ideal end state. If shortcuts or suboptimal trade off are desired, be explicit about it in comments.

ABP(Elsa): minimize total cost of ownership. Design something thoughtfully from the beginning.

Blog idea: Always write mod-level unit tests for each mod. Even if the code is super simple and the tests look boring. Having code that is hard to test/setup/validate is a smell that the code under test is not written well. Without tests, it will be possible to add non-trivial logic to the mod, and if there are no existing tests to extend, it will be harder to catch if we introduce code that is hard to test.

Blog idea: the key to thinking in dist systems is similar to real world: abandon the idea that there’s any concept of objective truth. There is only subjective truth. Perception is reality, but each perceiver has their own reality. Single host or single thread == perceiver. But then what? I don’t know what else I’d write about. The best way to scale is avoid having many perceivers having to synchronize on a shared state. Making decisions with local information rather than coordination is always best, if you can accomplish your goal with it. Local info is really about horizontal scalability. If your service has N hosts, and your system requires that O(N) interact to share data that is changing, then as you scale out, you will find a scaling cliff where all N hosts cannot interact effectively. <examples, connection mesh and concurrent connections limit, gossip delay causing data to always be too stale, N iteration to read all data>. You must partition your data so that sharing is only required among O(1) hosts. Then you can freaking scale!

Blog idea: Wiring pattern + functional tests

Blog idea: How to _Correctly_ use Dependency Injection Frameworks. Consider @Bean as publishing an object to a global namespace. Don’t publish every single object if it’s only used in one place. This post can be structured in identifying the problem/smell (e.g. bean created and used only in one place in same file), proposing to scope those things to a single method, and proposing the “full” solution of moving to wiring pattern [link to other post].

Blog idea: relationship with manager. Be honest about everything. If you don’t trust them or they don’t support you, get out.

gRPC error modeling convention

Copy over API best practices abstract from that one paper

structure APIs so there's only one right way to "plug in", similar to RAM plugging into a mobo.
