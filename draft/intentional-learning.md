---
layout: post
title: How to intentionally learn
---

How to you get better intuition? How do you train your judgement? How do you improve _how_ you learn?

How do you get smarter? (or is it "more smart", idk)

Here is one of the ways I go about being very intentional about learning. I'll call it Predict, Compare, and Recognize (PCR).

## (1) Predict / practice / attempt

Any situation where you review some existing work, like design documents or code reviews, you have a chance to practice judgement and develop some intuition.

Without further research beyond the document title, commit message, or backlog task description, think: what do you expect the solution to be? From your existing knowledge and the context you have about the software and the organization, try to solve the problem in your head.

Pay attention to where the tricky decisions are. Are there any gotchyas? Edge cases? Any ideas that initially seem attractive but end up being pitfalls? Any confusing terms? Any red-herrings/should-be-out-of-scope? If the problem is too large, break it down and start solving the sub-problems.

### Decision Branches

When exploring the decision space of the problem, pay attention to the decision branches. What type of database is used? Which component is responsible for behavior X? For each of these decision nodes, identify if this invalidates options from other decisions; i.e. which decision nodes are coupled.

_Side note: If you notice people artificially coupling two topics during a design discussion, it can be helpful to bring attention to how the two topics are independent, and we can discuss and choose each one in isolation._

### Fill in the gaps

If I don't know how some other dependent software component works, I usually assume it was built in the best, optimal way. I fill in the gaps with my best guess and then move on. So far, that has been pretty successful! It's not always perfect, sometimes I didn't have enough details, sometimes there is tech debt I wouldn't have a way of knowing about. But that's fine. You're not trying to get it perfect, you're trying to approximate gaps in your knowledge to keep moving forward. If you reach a point where you have a solution, that's when you circle back and check the assumptions you made.

### Fill in the gaps when decision branching

You can also use "fill in the gaps" when doing decision branching! Sometimes when predicting a solution, you're missing a detail of the problem, and there are multiple ways the solution could go depending on one or two choices in the problem definition.

Challenge yourself to make the choice that you feel is most likely to be true. Guess what you think will be most likely to be true based on what you know about the business. Then continue on with your mental problem solving.

Example: If an architectural decision depends on choosing if customers need a response in single digit ms vs hundreds of ms, and you're building a realtime app, you might decide we need a single digit ms response and then move forward with how to make that happen.

If there is, try to identify 2 different solution branches based on that outcome of that unknown.


### Force yourself to bring up one point

If you're having trouble with doing this for design review meetings, try to give yourself more time to think through your solution and compare to their solution. Read the document ahead of time. If there isn't one listed, ask the meeting owner for a link ahead of time, even if it's a rough draft. Or at least ask for a link to the backlog task/issue. Give yourself some time to read the content and understand it. Let it sink in. Now try to guess what will be the most interesting things talked about at the meeting. If you were forced to choose one topic to bring up and discuss, even just to talk through it and make sure you understand it, which topic would you bring up?

## (2) Compare / reflection / reverse branch

This is arguably the most important step to put mental effort into.



### Don't learn solutions in isolation

A solution only makes sense in the context of the problem it's solving. If you learn a solution without learning **what** problem it solves and **why** it solves it better than the other solutions in this context, then you're missing out on a lot of the learning and will likely be less effective in the next pattern recognition step.

A solution is a specific implementation choice. A problem is the thing forcing you to come up with a solution.

<TODO: link to Problem Solving Mind Set post>

???

### Persist the learning

## (3) Recognize

...as in "pattern recognition".

You've put effort into growing your number of real world examples to draw from, and hopefully the underlying fundamental concepts. Now, when you encounter new problems in the wild, pay attention to what sub-problems resemble problems you've seen before and identify solutions!

A huge part of being "smart" is, in my opinion, having good recall and good pattern recognition.

In my experience, this is also one of the hardest things to intentionally get better at, and unfortunately I'm unable to put a detailed method into words. If I had to suggest one thing, I'd say if you miss recognizing a pattern, ask yourself why you might've missed the pattern in this context, and what you could've done differently to recognize the pattern.

## Predict, Compare, and Recognize

Get out there and PCR! (make not cringey)

------

## Appendix - snippets

_Side note: If there are too many unknowns for a single review, and you are a more senior engineer reviewing a less tenured engineer's work, the right feedback might be to say the problem needs to be broken down further to be productively discussed as a team. And of course help talk through the problem breakdown process with examples, if help is needed._

CR/design review: anticipate the problem, design the high level solution, identify
where are the tricky parts where we might get something "wrong" in a way that impacts
customers/the business later.

Imagine how you'd solve something before attending meeting. Pay attention to differences.
Practice being the owner of all services/components/technology.
