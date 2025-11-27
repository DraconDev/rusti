i did investivate the css dead code situation and there are extensions that can help cleaning up unused css but they are mostly for web projects where you have html/js files to analyze together with css, in our case we only have css files so its not as straightforward, i will look into it more and see if we can find a solution that works for us, but most of all our solution doesn't even clean up unused files, only unused rules within files, so an extension is the solution here much like css peak 

also type check the css files we have and if any are unused then we mark what imports then with a warning


Feature 

Example:
<div --percentage="50%"> </div>

this should be valid css, we pass percentage variable to the css file and use it


ban ids?