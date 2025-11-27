i did investivate the css dead code situation and there are extensions that can help cleaning up unused css but they are mostly for web projects where you have html/js files to analyze together with css, in our case we only have css files so its not as straightforward, i will look into it more and see if we can find a solution that works for us, but most of all our solution doesn't even clean up unused files, only unused rules within files, so an extension is the solution here much like css peak 

also type check the css files we have and if any are unused then we mark what imports then with a warning





3. Enforce Accessibility (A11y)
This is the ultimate flex for a strict library. Since you are parsing the attributes anyway, validate them for correctness, not just existence.
The Rules:
<img /> without alt -> Compile Error.
<button /> with no text content and no aria-label -> Compile Error.
<a href="#"> (Empty link) -> Compile Warning.
<input> without a matching <label> (or aria-label) -> Compile Warning.
This moves you from "Type Safe" to "User Safe."

4. The "Unused CSS" Trap (The Reverse Check)
You mentioned you briefly had errors if the CSS file had unused classes.
Why you likely turned it off:
CSS is not just a static map. It has State and JavaScript.
The :hover Problem:
HTML: <button class="btn">
CSS: .btn { ... } .btn:hover { ... }
Strict Compiler: "Error: You never used class btn:hover in your HTML." -> FALSE POSITIVE.
The JavaScript Problem:
JS: document.querySelector('.menu').classList.add('open')
CSS: .menu.open { ... }
Strict Compiler: "Error: .menu.open is never used." -> FALSE POSITIVE.