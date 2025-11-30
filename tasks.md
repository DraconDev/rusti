### Tasks

## TODO

-   bug is that if we update the rust code then it correctly shows that that we have an error and the css is not found, but when we update the css file, the rust code is uneffected and we have to restart the server to make it work, evne when i change others sections it doesn't change again, cause it assumes it to be correct
-   even changing the very exact rust code then changing it back doesn't show that it's wrong,
-   ??? The css interop is a persisting problem that needs to be fixed, i dont want tailwind or inline styles cause that leads to hard to read code, so either we have css or style tags, but not both.
-   what other options are there? We can try to have classnames where the css is a variable like let section-header = style! { .section-header { color: "#fff" } }
-   one for sure inline styles and tailwind doesn't tell you what it supposed to be, only what styles it has
-   this does raise question about the script inclusion too,
-   i wonder if i should ease up and allow all kinds of css, but it hurts our value prop, albeit one can argue that people suppose to make components , also i want to cut down on the checking to not hurt compile times
-   css is not that safe i can change it to all kinds of nonsense like

```css
.lesson0-container {
    min-height: 1 00vh;
    background: linear-gra dient(135deg, #0f172a 0%, #1e1b4b 100%);
    padding: 2rem;
    max-width: 120 0px;
    margin: 0 au to;
    position: relati ve;
    overflow-x: hid den;
}
```

##

-   Style all lessons in a modern dark theme
    -   [x] style lesson 0
    -   [x] style lesson 1
    -   [x] style lesson 2
    -   [x] style lesson 3
    -   [x] style lesson 4
    -   [x] style lesson 5
    -   [x] style lesson 6
    -   [x] style lesson 7
    -   [x] style lesson 8
    -   [x] style lesson 9
    -   [x] style lesson 10
    -   [x] style lesson 11
    -   [x] style lesson 12
    -   [x] style lesson 13
    -   [x] style lesson 14
    -   [x] style lesson 15
    -   [x] style lesson 16
    -   [x] style lesson 17
    -   [x] style lesson 18
    -   [x] style lesson 19
    -   [x] style lesson 20
    -   [x] style lesson 21
    -   [x] style lesson 22
    -   [x] style lesson 23
    -   [x] style lesson 24
    -   [x] style lesson 25
    -   [x] style lesson 26
    -   [x] style lesson 27
    -   [x] style lesson 28
    -   [x] style lesson 29
    -   [x] style lesson 30
    -   [x] style lesson 31
    -   [x] style lesson 32
    -   [x] style lesson 33
    -   [x] style lesson 34

## Done
