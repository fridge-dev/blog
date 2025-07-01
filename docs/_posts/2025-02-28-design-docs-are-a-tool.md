---
layout: post
title: Design docs are a tool
---

**Design docs are a tool to save you time.** If you hate writing design docs and procrastinate doing them, here is some philosophies that fixed it for me.

### When should you write a design doc?

Let's start with _**Why are you writing a design doc?**_ Or rather, why don't you just go start building?

The risk of going straight to coding is that you could waste a lot time in the long run. You could spend days coding before discovering an integration incompatibility. You could spend dozens of human-years successfully building and maintaining the wrong thing. Even churn and small change in plans at code review time can be surprisingly costly. The cost of failure/iteration at execution time can be super high.

The purpose of a design is to discover these risks when it's cheap to fail and iterate.

### When should you NOT write a design doc?

You don't need a design doc if it's less costly to build, fail then iterate.

I know it sounds obvious, but it's helpful to compare this to work that does require a design to remind yourself why you're bothering to write a design at all. Changes with small scope and miniscule impact almost never require a design.

# Philosophy

### Anti-pattern - Perfection

Don't treat your design doc as a precious and sacred artifact. Don't attemt to completely define every single minute aspect of the problem and solutions just to flex. Don't fantasize that future generations will cherish your examplary doc as a spectacle of quality writing. These are all traps.

Polishing a doc has diminishing ROI days earlier than you might think.

TODO: where?
A design should be considered "complete" when it gives the enough confidence needed for the given business context. Generally, as the downstream impact of getting it wrong increases, the required confidence should go up.

## Anti-pattern - Designing in secret

Getting feedback early and often is super important to make sure you're not wasting time going in the wrong direction! Don't fall for the trap of drafting everything in isolation, imagining how cool the ✨big reveal✨ will feel, and delaying the first team review.

Get some eyes on an incomplete draft, even if it's just one person. Or better yet, sit down with someone and draw your idea on paper or whiteboard. Ask for feedback on if it sounds sane and what major areas are concerning. Don't be afraid of getting critical feedback. If you're writing about the wrong thing, it's better to get that feedback now than after days of polishing unneeded details.

Try to value the pride in getting to the right outcome __quickly__ higher than the pride of getting to the right outcome __by yourself__.

## Anti-pattern - Waiting to schedule the review meeting

If you are thinking about scheduling a design review, it's probably time to send a design review invite. Schedule it 2-3 days from now, and force yourself to get a mostly complete, good enough document by that time.

Set the expectation that your doc is intentionally scrappy. Write in the meeting invite that you might not have a fully complete proposal, but you don't want to delay initial feedback.

#### Tip - How to draft for early feedback

One strategy I like to do is put placeholder sections and solution options with "TODO" bullet points to show roughly what I'm planning to write about, even if there are open questions and unknowns to evaluate. Ask a pre-reviewer if the doc structure makes sense and if the planned content will be useful to help make a decision. This helps you spend iteration time where it matters and you can avoid wasted time being thorough where it's not needed.


## Anti-pattern - Not failing fast enough

Your goal is to fail fast! That means draw attention to the topic most likely to cause your idea to fail.
You should be trying to

### Tip - Design should feel scrappy

It's okay for design to feel scrappy! Avoid over-polishing. When you get consensus on your proposal, you should feel like there was more things you wanted to tweak or small things you wanted to add. If that's not how you feel, you may have gotten to the point of diminishing returns.

An pretty good doc today is usually better than a great doc a week from now.

As a thought exercise, imagine what if everyone is going to agree with what you're trying to write, and the only thing holding you back from starting implementation is getting it written down and reviewed?

### Tip - Design docs are important

Don't get me wrong, writing out your ideas on (digital) paper is a super helpful strategy in both making sure you understand your idea and making sure you are solving the correct problem. I am simply suggesting a new lens to view the designs you may be dreading writing.

### Tip - When to invest in a design doc

Sometimes design docs can save you time beyond implementation. Think about total cost of ownership and long term ROI (return on investment).

There are situations when it's justified to spend the extra effort on a document. For example, if you expect your proposal will be contentious and require multiple rounds of discussion, if your doc has a large audience, if your doc will be the authoritative documentation of a complex area of the system, if your doc is the high level entry point into a project, if your doc contains re-usable evaluation of a common algorithm or technology, etc. However, these should be considered exceptions, not the norm.

## Anti-pattern (of not writing) - Mis-estimating cost of iteration

It's also very easy to under-estimate the total cost of iteration at execution time. Execution phase can have many steps: prep-refactor, define APIs, write code docs, implement code, more code docs, don't forget telemetry, add proper error handling, write unit tests, debug unit tests, refactor your new code, write integration tests, all with code reviews, and possibly including deployments if there are cross-team API changes. It's also super likely that new ongoing maintenance/operations cost will creep in. Did your change invalidate existing documentation that's now stale?

There's also downstream impact to like loss of customer trust, slow time to market, missed opportunity. Building the wrong thing can cost way more than just writing the wrong coding.

### Design ROI

Simply put, the design input effort should be aligned with the output value you're getting from it. Think about the outcome you want, and what will be the cheapest way to get there considering the total cost of changing your mind, aligning the team, implementation, testing, and long term ownership when applicable.

A design doc is not a form to fill out. Be fluid, be pragmatic, use your judgement about what the right amount of detail is for the task. Good luck!
