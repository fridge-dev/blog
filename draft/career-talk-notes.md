Career talk notes

==== Intro ====

- Writing is not just for fiction/art! Lack of communication (text/verbal) will be a huge career limiter.
- Clearly writing, communicating, and thinking are all equivalent. Improving your written communication will improve your thinking/intelligence.

What are you working on?
    - Finished writing a dozen various docs for Lambda Lite/invoke routing
        - "Loves writing docs"
    - Coding multi-concurrency load-balancing and adaptive scaling

==== Before Writing ====

Epiphany: Ambiguity / idea
    - Problem solving, break down the problem into smaller more management parts
    - In the history of Amazon, super hard problems have been solved. Whatever problem you have will be solvable with help of your team.
    - One weird technique that helps my thought process if I'm overwhelmed and my brain is racing all over the place with various thoughts is I will type my thoughts in my phone notes app as I think. There's something about the pace of my typing, and how iOS it's really clunky to go back and edit/wordsmith things, so you're less focused about exactly how you're phrasing something and you just write out your thoughts. And it's super useful to have a physical, replayable version of my thoughts.
        - This helped me have a breakthrough in two really hard technical designs, one for SbES ordering and one for Appulse MC replication

Respect what comes before
    - Don't think you're smarter than the previous generation just because you have more context on how things evolved. Trust that the architects of the past were most likely super smart and made the choice that was right for the situation/context.
        - Acknowledge that premature optimization/abstraction is generally bad, and past architects might've been leaving room for you to choose how to evolve the service.
        - One thing I _will_ criticize is if unusual choices were made without fully writing that context somewhere that's discoverable, but it's still usually pretty easy to rationalize how past decisions were made.
    - E.g. WorkerManager & ScalingArcs, we didn't know how successful Lambda would be, we didn't appreciate how costly capacity outages can be;
        - The past generation built a performant and horizontally scalable service that helped successfully take Lambda from 0 to millions in revenue.
        - It's easy to criticize and say "the code was complicated" or "ring blocklisting was catastrophic", but IMO WM was a success.

==== Doc Writing ====

Epiphany: How to wrote a design?
    - Crank out content, epiphany is usually what type of diagram to draw, or how to order the content if there are inter-related problems
    - Go whiteboard the idea with your peer (serves as a sanity check too).
        - Pay attention to what order you give background/new info. When do you reach for marker to draw? What do you draw?

How to start writing?
    - Classic Lambda answer: You've gotta understand what problem you're solving. Write a good problem statement.
    - Alt state of mind: Write down whatever ideas you have in any unordered fashion.
    - Starting block? It helps me to temporarily forget anyone else is going to read this. Pretend I'm just going to YOLO/solo the project. What is stopping me from starting impl? I don't know exactly what to build. Write out whatever I need to help me decide.

How to revise?
How to write “easy to follow” doc? Secret writing tips?
    - Philosophy
        - Empathy - Instead of doing what’s easy for you, do what’s easy for your reader.
            - Be respectful of readers' time and attention span.
        - Unselfish perspective - I’m not writing this doc because I’m important. I’m doing it because _you’re_ important.
    - Practical
        - Don't bury the lead - state conclusion up front, supporting details later. Unless you're writing a detective story.
        - Breadth-first concepts (10k view, 1k view, 100 view).
            - Break out to dedicated docs - If you feel like there's too much to cover, it likely means you need alignment on the higher level strategy and separate, dedicated design docs.
        - Move stuff to Appendix!
            - Sunk cost fallacy - If you put in the effort and feel bad deleting it, just move it below-the-fold. Reference it from main doc.
            - Feels super liberating. Keeps core doc concise.
    - ROI/Polish
        - Put in the effort that it gives you value

Have you drastically change direction in middle of the writing?
    - Don't be afraid to put super detailed but discarded ideas in the Appendix. It helps show future generations what you were thinking.

How do you handle the doc when you feel the scope is exploding?
    - Get content down on paper so you don't have to hold it in your brain. Doesn't have to perfect.
    - Once you have raw, unordered content,
        - I haven't had a scope explosion

What are important skills/tools to use to write a good design doc?
    - Drawing diagrams on paper! Super underrated skill. Improve your own understanding of the problem.
        - Force yourself to diagram things in a quick, sloppy way. Plan to not show anyone (removes the expectation of quality/understandability)
    - Sequence diagrams - force you to understand ordering, branching, error handling, identify new problems, iterate way faster than code
    - Diagrams
        - easier to grok quickly
        - rule of thumb: 1 diagram per large section
        - don't force it
        - empathize with your readers, no one likes a thick wall of text

==== After Writing ====

How to drive a review meeting?
    - Before meeting: Role play. What if you didn't need any review/consensus. Are you confident you're building the right thing?
        - What things do you feel uncomfortable about? What things do you think someone might disagree with? Write those down in an informal bullet point list. Include it in the doc in a "Ambiguity/Contention" section.
    - Stay focused
        - Politely interrupt if topic is getting off-track, state that that topic may be important and can follow up later, you're not seeking feedback on that right now

[TODO ADD TO QUIP] How do you retrospect/learn from your writing?
    - Pay attention to things that people find confusing
    - Read your own docs from years past. How quickly do you understand it?

==== Be a Reviewer ====

How do you judge other's design?
    - Read other docs to learn things you like and don't like
    - Helps empathy
    - If you're confused, try to understand how you might've written it better and apply that to your own writing
    - If you understood something, try to understand what you like about it

What data point/information you typically look for when you review a doc?
    - Do I understand the problem, independent of any solution.
        - If we decided to do nothing, what would the customer problem be? What would the ops/maintenance cost problem be?

== Questions to add ==

What's a common pitfall of the design process?

What's an underrated design habit/practice?
