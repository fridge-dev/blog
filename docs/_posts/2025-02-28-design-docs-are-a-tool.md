---
layout: post
title: Design docs are a tool
---

**Design docs are a tool to save you time.** If you're not treating it that way, try on this lens.

I've heard peers say they hate writing docs, they're bad at it, it's stressful, they procrasinate. I mildly share the same feeling. Here is my philosophy for writing docs that has helped me and hopefully can help you.

### Anti-patterns

Don't treat your design doc as a precious and sacred artifact. Don't flesh out all minute details of all conceivable alternatives just to show how much you've thought about it. Don't fantasize that future generations will cherish your examplary doc as a spectacle of quality writing. These are all traps.

### When should you write a design doc?

Why are you writing a design doc? Or rather, why don't you just go start building?

The risk is that you waste time. If you go straight to coding, you could spend days before discovering a fundamental flaw in your idea that maybe could've been caught by a group review. You could run into significant churn during code reviews when having to answer numerous questions back and forth to explain the bigger picture of your changes. You could learn during code review that there's a way easier or more sensible way of achieving your goal, and you just spent days writing throw-away code (and tests!). Or arguably worst of all, you could've misunderstood the problem and build a valid solution to the wrong problem.

You could also repeat and experience multiple of these drawbacks for a single project.

The purpose of a design is to save you this time by discovering these issues when it's cheap to fail. For large projects, it's way cheaper to iterate on a written design than code.

### When should you NOT write a design doc?

For trivial, straight-forward tasks, you usually don't need a design.

Think about the total cost of the change. If implementation, testing, peer review, iteration, etc. are cheaper than the typical cost of writing and reviewing a design, then a design doc is likely not needed. Making trivial code changes, writing a 20-line bash script, fixing a flakey test, upgrading a dependency, refactoring a method signature, changing an automation threshold, choosing which IDE to install, changing a team meeting time, ... if the cost of going straight to implementation, review, and iteration are cheaper than the time it would take to write a design doc, the design is probably not worth the time.

I know it sounds obvious, but it's helpful to compare this to work that does require a design to remind yourself why you're bothering to write a design at all.

### Get feedback early and often

Getting feedback early and often is super important to make sure you're not wasting time going in the wrong direction!

Another anti-pattern is to draft everything in secret and isolation, imagining how cool the ✨big reveal✨ will feel when you first share your completed document with your peers. Don't do that. Get some eyes on an incomplete draft, even if it's just one person. Ask for feedback on if you're headed in a good direction and if a team review in a couple more days will be a good use of time with the content you're planning to add. Don't be afraid of getting critical feedback. Fail fast! If you're writing about the wrong thing, it's better to get that feedback now than after days of polishing unneeded details.

I know there's a certain pride of doing it yourself. But there's also a pride in getting stuff done quickly and consistently. And your teammates and leadership will take notice if you're consistently producing high quality work with a fast turn-around time. Using your teammates effectively to get work done is a strength, not a weakness.

#### How to draft for early feedback

One strategy I like to do is put placeholder sections and solution options with "TODO" bullet points to show roughly what I'm planning to write about, even if there are open questions and unknowns to evaluate. Ask a pre-reviewer if the doc structure makes sense and if the planned content will be useful to help make a decision. You never know, maybe one of your early-reviewers can help save you a time by telling you one option is definitely not acceptable, and you don't need to thoroughly evaluate it. This way you spend iteration time where it matters and you can save time going into detail where it's not needed.

### Design should feel scrappy

It's okay for design to feel scrappy! When you get consensus on your proposal, you should feel like there was more polish and word-smithing you wanted to do, or maybe there were still a couple of small sections you wanted to add. If that's not how you feel, you may have gotten to the point of diminishing returns.

As a thought exercise, what if you put in the bare minimum effort to get your design proposal accepted? (I don't mean to write incoherently or to misguide a decision to the fastest outcome. Remember, if you regret the choice in the long term, then your misguided design will likely cost you more total time.)

### When to invest in a design doc

Design docs can save you time beyond implementation. Think about total cost of ownership and long term ROI.

There are situations when it's justified to spend the extra effort on a document. For example, if you expect your proposal will be contentious and require multiple rounds of discussion, if your doc has a large audience, if your doc will be the authoritative documentation of a complex area of the system, if your doc is the high level entry point into a project, if your doc contains re-usable evaluation of a common algorithm or technology, etc. However, these should be considered exceptions, not the norm.

### Design ROI

Simply put, the design input effort should be aligned with the output value you're getting from it. If it's a decision with a smaller impact, it's okay to write a very hasty 1-pager! And a "large" decision doesn't always need an overly "large" doc. Think about the outcome you want, and what will be the cheapest way to get there considering the total cost of changing your mind, aligning the team, implementation, testing, and long term ownership when applicable.

A design doc is not just a form to fill out. Be pragmatic and use your judgement about what the right amount of detail is for the task. Good luck!
