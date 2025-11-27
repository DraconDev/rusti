i did investivate the css dead code situation and there are extensions that can help cleaning up unused css but they are mostly for web projects where you have html/js files to analyze together with css, in our case we only have css files so its not as straightforward, i will look into it more and see if we can find a solution that works for us, but most of all our solution doesn't even clean up unused files, only unused rules within files, so an extension is the solution here much like css peak 





3. Enforce Accessibility (A11y)
This is the ultimate flex for a strict library. Since you are parsing the attributes anyway, validate them for correctness, not just existence.
The Rules:
<img /> without alt -> Compile Error.
<button /> with no text content and no aria-label -> Compile Error.
<a href="#"> (Empty link) -> Compile Warning.
<input> without a matching <label> (or aria-label) -> Compile Warning.
This moves you from "Type Safe" to "User Safe."

