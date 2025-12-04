# TODO


- enforce all css to be double quoted
- variables names should be valid for style blocks 
            --azumi-bg: "#1e1e1e";
            --azumi-text: "#fff";
            --azumi-text-dim: "#ccc";
            --azumi-primary: "#6366f1";

# Maybe

   <div class={fill} style="--progress: {completion}; --accent: {accent_color}"></div>
 vs ?
   <div class={fill} style={--progress: completion; --accent: "red"}></div>
?
  <div class={fill} style={--progress: completion; --accent: format!("{}", accent_color)}></div>

# Later

- hot reload 
- also what nextjs features we could implement?
  - what features we are missing 
- action testing 
- maybe improve css validation 
- image optimization
- make the lessons

# Done

- how to handle global?
  - we would also need a global tag for the style <style global>
- snake case for css?
- list-style should be list_style

