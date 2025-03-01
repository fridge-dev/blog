---
layout: post
title: Design docs are a tool
---

**Design docs are a tool to save you time.** If you're not treating it that way, you're doing it wrong.

I've heard from numerous people that they hate writing docs, it's stressful, they're bad at it, they procrasinate on it. I mildly share the same feeling. Here is my philosophy for writing docs that has helped me and hopefully can help you.

### Anti-patterns

Don't treat your design doc as a precious and sacred artifact. Don't flesh out all minute details of all conceivable alternatives just to show that you've thought things through. Don't fantasize that future generations will cherish your examplary doc as a spectacle of quality writing. These are all traps.

### When should you write a design doc?

Why are you writing a design doc? Why don't you just go start building?

The risk is that you waste time. While coding, you could discover a fundamental flaw in your idea that could've been caught by a group review. You could run into significant churn during code reviews when having to explain the bigger picture of your intention. You could learn during code review that there's a way easier or more sensible way of achieving your goal, and you just wasted days writing throw-away code (and test!). Or arguably worst of all, you could straight up build a solution to the wrong problem.

The purpose of a design is to save you time in the long term. For large tasks, it's way cheaper to iterate on a design, including throwing away work, than doing the same while coding.

### When should you NOT write a design doc?

If implementing an idea, testing it, rolling it back, etc. is cheaper than the typical cost of the design process, then a design doc is likely not needed. Making trivial code changes, fixing a flakey test, upgrading a dependency, changing a team's meeting time, choosing which IDE to install, ...

I know it sounds obvious, but it's helpful to compare this to work that does require a design to remind yourself why you're bothering to write a design at all.

### Get feedback early and often

Getting feedback early and often is super important to make sure you're not wasting time!

Another anti-pattern is to draft everything in secret and isolation, imagining how cool the ✨big reveal✨ will feel when you first share your completed document with your peers. Don't do that 😅. Get some eyes on an incomplete draft, ask for feedback on if you're headed in the right direction. Don't be afraid of getting critical feedback. Fail fast! If you're way off, it's better to get that feedback now than days or weeks of drafting.

I like to put placeholder sections and "TODO" bullet points to show roughly what I'm planning and ask if the doc structure makes sense and if the details I'm planning to get will be useful to help make a decision. You never know, maybe one of your early-reviewers can help save you a ton of time by telling you one option is definitely not acceptable, and you don't need to thoroughly evaluate it. This way you spend iteration time where it matters and you can save time going into detail where it's not needed.

### Design should feel scrappy

Most of the time, design should feel scrappy. When your design gets accepted, you should feel like there was more polish and word-smithing you wanted to do, or maybe there were still a couple of details you wanted to add. If that's not how you feel, you may have gotten to the point of diminishing returns. But don't give in to that temptation. Get on with building what you've designed!

As a thought exercise, what if you put in the bare minimum effort to get your design proposal accepted? (I don't mean to write incoherently or to misguide a decision to the fastest outcome. Remember, if you regret the choice in the long term, then your design might not have saved you time.)

### When to invest in a design doc

Design docs can save you time well beyond the initial implementation of the design choice. Think about total cost of ownership and long term ROI. Through the same lens of saving time, there are situations when it's justified to spend the extra effort on an otherwise acceptable document.

However, these should be considered the exception, not the norm.

1. If you plan to frequently reference some data or writing during development, it might be useful to spend extra time organizing your document to be readable.
1. If your doc is a design for a complex area that frequently requires people to reference the design rather than code, it could be worth doing a minor revision after implementation to make sure the doc is accurate.
1. If your doc is the high level entry point into your team's domain, it is probably worth the time to polish and make the doc readable to a broader audience.
1. If your doc contains evaluation of a common algorithm or technology, it may be re-usable by other teams for their hypothetical future work.

### Design ROI

Simply put, the design input effort should be aligned with the output value you're getting from it. If it's a decision with a smaller impact, it's okay to write a very small design doc! But a large decision doesn't always need a large doc to help illuminate the way forward. Think about the decision you're trying to make and the ROI (return on investment) of your drafting time. But make sure to consider the long term ROI and total cost of ownership.

A design doc is not a form to fill out. Don't just go through the motions. Use your judgement and write what you think is needed. Be pragmatic. Treat it as a means to an end. Good luck!
