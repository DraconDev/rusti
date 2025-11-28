i did investivate the css dead code situation and there are extensions that can help cleaning up unused css but they are mostly for web projects where you have html/js files to analyze together with css, in our case we only have css files so its not as straightforward, i will look into it more and see if we can find a solution that works for us, but most of all our solution doesn't even clean up unused files, only unused rules within files, so an extension is the solution here much like css peak 

mention that we also support fragments <> inthe readme


<!-- wait before we proceed regarding lesson 14 we do have a possible bug that what you create the rust file write in it, then create teh css we are getting 

ailed to read CSS file '/home/dracon/_Dev/_Fun/azumi/demo/static/pages/lesson14.css': No such file or directory (os error 2)rust-analyzermacro-error
azumi_macros
proc_macro html

now i think it work but our rust analyzer shows it as an error  -->


one but that if we make a css file rust analyzer doesn't realize we have it, and keeps showing it as an error, i think we need to add a custom rule to our rust analyzer to ignore this, or not sure what to do