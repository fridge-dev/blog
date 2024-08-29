1. Future prevention / automation
    - scoped to automated alerts that ticket humans?
        Yes. That will limit the audience a bit, but will make a more
        impactful point to those it applies to. And I'm not trying to
        influence hundreds of thousands of engineers. Just a few dozen.
2. How to teach/give feedback
    - apply future prevention to teaching moment.
        - ACK: sometimes, re-teaching will be unavoidable
    - how to not "fix" this outcome, but allow the person to never make
        this same choice/mistake again
    - later: generalize to pattern recognition
    - understand there's a human brain working through the problem solving
        flow. You must find out how people think, what steps they take, and
        then figure out how to inject the practice/principle into their mental
        problem solving execution
3. The career growth choice you didn't know you had at Amazon
    - I'm often asked how do I grow to the next level? How do I get promoted?
    - I'm perfectly happy to help in these situations. I admire self-growth
        and encourage those passionate about it to keep pursuing it.
    - Before I answer, I always make sure that both of us acknowledge something
        important: It's okay/acceptable to not want to grow. It's okay to not
        want to grow as fast as possible. And it's a choice between career growth
        and whatever else one can do with their life.
    - You're at amazon, so you must be good at solving problems. And there's
    readily available opportunity in front of you. If you're good at it, you enjoy it,
    and there's opportunity, then it can be an obvious choice to go all-in.

    But there's one thing that takes a higher level of consciousness and a longer term
    thinking about. What is the opportunity cost of you investing that energy.

    It's easy at amazon to get pulled into the high speed current and see everyone
    else around you swimming as fast as they can and assume that that's the best way
    to live life. This isn't to discourage you from trying to swim as fast as possible.
    Just make sure you pop your head out of the water and make sure you know where you're
    swimming, why you're swimming there, where else you could be swimming, and that you're
    happy with all of it.



4(?). Rust state machine impl - do you always need a wrapper type with Option<SM> where you .take() and then put it back?
    - do we pretty much always need a Holder<T> type of abstraction?
    https://geeklaunch.io/blog/make-invalid-states-unrepresentable/

Other team/soft topics:

- Pretend you're the CTO of this small startup team/org. What would you do if you
had to choose? Would you choose A or B, or choose to spend more effort gathering
info? What would you do to gather more info? What if time was no constraint? What
if money were no constraint? What if you had very


Learning: Ask yourself "why". Ask why about choices you make, choices others have made.
Keep asking why, and if you get an answer that doesn't make sense, then keep chasing.
There is probably some interesting, novel new discovery for you to learn. Be able to
distill it down to its purest most raw form. Does that choice make sense to you? If not,
then keep asking why. You'll invetiably reach a point where you don't understand/recognize
the words or the ideas being talked about, so you'll have to shift to "what is X"
/ "how does X work" mode, but then get back to the why.

CR/design review: anticipate the problem, design the high level solution, identify
where are the tricky parts where we might get something "wrong" in a way that impacts
customers/the business later.

Generalize lambda design doc: Always ask "what problem does this solve". If it doesn't solve
any, then it most likely doesn't have a reason to exist. Some exceptions are "we think it
might solve this problem" or "we don't yet know which problem this solves"... i.e.
marketing/directional/existential prediction, then yeah, okay. But from an engineering perspective,
it doesn't have validity.

Input goals vs output goals

How to keep a service "dumb" and simple: implement policies/mechanisms, but don't select the policy.
>I think it’s important for keeping Component A simpler that Component A is not doing some policy selection based on request params, but instead just lets the client select the policy explicitly in the request. Having service implement the mechanism for different policies, and client select which policy to use is a pretty common pattern that’s useful for keeping the server simpler and more focused on “how to do X, how to do Y” and not “when should I do X vs Y”.
>Component B has this same pattern in a few interactions with other components. It helps simplify Component B a lot, which is important because even just implementing behavior X can be quite complicated, and we prefer (when it makes sense) to outsource complex business logic of choosing when/why to do X.

Imagine how you'd solve something before attending meeting. Pay attention to differences.
Practice being the owner of all services/components/technology.
