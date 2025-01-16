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


3. The career growth choice you didn't know you had
    - I'm often asked how do I grow to the next level? How do I get promoted?
    - I'm perfectly happy to help in these situations. I admire self-growth
        and encourage those passionate about it to keep pursuing it.
    - Before I answer, I always make sure that both of us acknowledge something
        important: It's okay/acceptable to not want to grow. It's okay to not
        want to grow as fast as possible. And it's a choice between career growth
        and whatever else one can do with their life.
    - You're at a top tier tech company, so you must be good at solving problems. And there's
    readily available opportunity in front of you. If you're good at it, you enjoy it,
    and there's opportunity, then it can be an obvious choice to go all-in.

    But there's one thing that takes a higher level of consciousness and a longer term
    thinking about. What is the opportunity cost of you investing that energy.

    It's easy in competitive tech companies to get pulled into the high speed current and see everyone
    else around you swimming as fast as they can and assume that that's the best way
    to live life. This isn't to discourage you from trying to swim as fast as possible.
    Just make sure you pop your head out of the water and make sure you know where you're
    swimming, why you're swimming there, where else you could be swimming, and that you're
    happy with all of it.

    Other advice:
        - Take care of your body. Excercise! Go to the gym, play sports, don't feel guilty to splurge and spend money on XYZ if it will make it more likely for you to do physical activity. You'll be a lot happier with life if you're healthy.
        - Don't take things too seriously. Big tech can feel high stakes, intense, sometimes competitive. If that is stressing you out, remember that it's just a job. You'll have a long career.
        - Be patient with yourself. Give yourself time to learn. Give yourself permission to not know everything your coworkers know. Everyone learns differently. You will get there, it just takes time.




4. Rust state machine impl - do you always need a wrapper type with Option<SM> where you .take() and then put it back?
    - do we pretty much always need a Holder<T> type of abstraction?
    https://geeklaunch.io/blog/make-invalid-states-unrepresentable/

Other team/soft topics:

Learning: Ask yourself "why". Ask why about choices you make, choices others have made.
Keep asking why, and if you get an answer that doesn't make sense, then keep chasing.
There is probably some interesting, novel new discovery for you to learn. Be able to
distill it down to its purest most raw form. Does that choice make sense to you? If not,
then keep asking why. You'll invetiably reach a point where you don't understand/recognize
the words or the ideas being talked about, so you'll have to shift to "what is X"
/ "how does X work" mode, but then get back to the why.
Five whys (https://aws.amazon.com/blogs/mt/why-you-should-develop-a-correction-of-error-coe ):
    - Ask why, root cause, don't assume, tirelessly don't assume
    - not just LSEs/COE/5y;
    - ask why a service behaves the way it does. Ask why a new feature/design is the right thing to build.
    - when you learn a new concept, ask yourself what problem does it solve and why? What problems does it NOT solve?
    - help apply in future
    - Also discover historical decisions/debt of team
Learning vs delivering:
    - Do both, just know when it's the right time to do which
    - Know your learning style
    - Learn by practice? Deliver lots
    - Otherwise:
        - Deliver more slowly, but learn the peripheral content deeply
        - Learn what you're interested in!
        - Especially in your first few years
        - Communicate with your manager about expectation, balance learning with urgency


CR/design review: anticipate the problem, design the high level solution, identify
where are the tricky parts where we might get something "wrong" in a way that impacts
customers/the business later.

Imagine how you'd solve something before attending meeting. Pay attention to differences.
Practice being the owner of all services/components/technology.

Generalize design doc: Always ask "what problem does this solve". If it doesn't solve
any, then it most likely doesn't have a reason to exist. Some exceptions are "we think it
might solve this problem" or "we don't yet know which problem this solves"... i.e.
marketing/directional/existential prediction, then yeah, okay. But from an engineering perspective,
it doesn't have validity.
- When beginning a new task/project, always understand WHAT the problem is, not HOW you're solving it.
- Don't get tunnel-vision on the solution.
- Always keep the problem statement in mind.
- If we don't solve this, what problem will exist?
- If you don't understand the problem, ask your SDM or tech lead.
- Helps you:
    - come up with new solutions when you hit blockers
    - make decisions about tradeoffs
    - choose acceptable, simplified solutions
    - avoid boiling the ocean
- Choosing to "do nothing" to solve a semi-related problem is always a 2-way door
Example (oversimplified):
- social shopping app in US customers tagging products in uploaded images
- global CDN service based on internal S3 buckets
- Building 2nd stack in DUB
- "Onboard our DUB S3 bucket to CDN service"
- CDN service only supports buckets in us-east-1
- => what's the problem?
    - Customer images need to be put into CDN
    - Customers need to upload images to a local S3 bucket
- Solution: Customer uploads the image to DUB S3, when customer hits "post", async workflow copies the image from DUB to IAD


How to keep a service "dumb" and simple: implement policies/mechanisms, but don't select the policy.
>I think it’s important for keeping Component A simpler that Component A is not doing some policy selection based on request params, but instead just lets the client select the policy explicitly in the request. Having service implement the mechanism for different policies, and client select which policy to use is a pretty common pattern that’s useful for keeping the server simpler and more focused on “how to do X, how to do Y” and not “when should I do X vs Y”.
>Component B has this same pattern in a few interactions with other components. It helps simplify Component B a lot, which is important because even just implementing behavior X can be quite complicated, and we prefer (when it makes sense) to outsource complex business logic of choosing when/why to do X.


To learn a new code base: refactor it. Don't check it in. Just restructure the existing ideas in your own mental model. This requires you to deeply understand what's going on and what are the tricky edge case behaviors.
This also requires you to be good at refactoring correctly (i.e. no unintended behavior change), which is prob worth another more detailed post about how to be good at refactoring. This also requires you to be opinionated (not necessarily objectively good) at coding, as it requires you to see some functionally correct code in form A and think "nah that's not how i'd do it" and rewrite it in form B.
These don't need to be large refactors. My most common example are changing/adding/removing encapsulation layers and changing public APIs to more simply model the current usage and remove any premature abstractions [TODO link to premature abstractions post?](...).
Take it from Cal Newport: QEC note habit: to understand something, you should rewrite your note in your own structure [TODO citation, see https://www.youtube.com/watch?v=5O46Rqh5zHE].
