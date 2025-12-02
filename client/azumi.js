class Azumi {
    constructor() {
        this.scopes = new WeakMap(); // Element -> Proxy
        this.delegate();
    }

    // Event delegation
    delegate() {
        ["click", "submit", "change", "input"].forEach((event) => {
            document.addEventListener(event, (e) => this.handleEvent(e));
        });
    }

    // Parse az-on attribute
    handleEvent(e) {
        const target = e.target.closest(`[az-on]`);
        if (!target) return;

        // Check if the event type matches the trigger (e.g. "click ...")
        // Simple parsing: "click call foo" or "submit call bar"
        const attr = target.getAttribute("az-on");
        const parts = attr.split(" ");
        const trigger = parts[0];

        if (trigger !== e.type) return;

        e.preventDefault(); // Default prevent for handled events

        // Parse the rest: "call toggle_like -> #box"
        // This is a very basic parser for the prototype
        const action = this.parseAction(parts.slice(1).join(" "), target);
        if (action) this.execute(action, target);
    }

    parseAction(cmd, element) {
        // Format: "{event} call {action} -> {target} {swap}"
        // or "{event} set {key} = {value}"
        // NOTE: TokenStream adds spaces around punctuation, so "-> #id" becomes "- > # id"

        // Remove extra spaces and reconstruct operators
        cmd = cmd.replace(/\s*-\s*>\s*/g, "->").replace(/\s*#\s*/g, "#");

        const tokens = cmd.split(" ");
        const eventType = tokens[0]; // "click", "submit", etc.
        const actionType = tokens[1]; // "call" or "set"

        if (actionType === "call") {
            let actionName = tokens[2]; // The actual action function name
            let url = `/_azumi/action/${actionName}`;
            let targetSelector = null;
            let swap = "morph";

            const arrowIndex = tokens.indexOf("->");
            if (arrowIndex !== -1) {
                targetSelector = tokens[arrowIndex + 1];
                if (tokens[arrowIndex + 2]) {
                    swap = tokens[arrowIndex + 2];
                }
            }

            return { type: "call", url, target: targetSelector, swap };
        }

        // TODO: Implement 'set' for local state
        return null;
    }

    // Execute: "call toggle_like -> #box" or "set open = true"
    async execute(action, element) {
        if (action.type === "call") {
            await this.callAction(action, element);
        } else if (action.type === "set") {
            this.setState(action, element);
        }
    }

    // Server action
    async callAction(action, element) {
        // Collect form data if it's a form or has inputs
        let body = null;
        if (element.tagName === "FORM") {
            body = new FormData(element);
            // Convert to JSON for Axum if needed, or send as FormData
            // For now, let's assume JSON payload for actions
            const data = Object.fromEntries(body.entries());
            // Add context from az-scope if needed
            body = JSON.stringify(data);
        } else {
            // For buttons, get state from az-scope on nearest parent with az-scope
            let scopeElement = element.closest("[az-scope]");
            if (scopeElement) {
                let scopeData = scopeElement.getAttribute("az-scope");
                body = scopeData || "{}";
            } else {
                body = "{}";
            }
        }

        try {
            const res = await fetch(action.url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body,
            });

            if (!res.ok) throw new Error(`Action failed: ${res.status}`);

            const html = await res.text();

            let target = element;
            if (action.target) {
                target = document.querySelector(action.target);
            }

            if (target && window.Idiomorph) {
                window.Idiomorph.morph(target, html);
            } else if (target) {
                console.warn("Idiomorph not loaded, falling back to innerHTML");
                target.innerHTML = html;
            }
        } catch (err) {
            console.error("Azumi action error:", err);
        }
    }

    // Local state change
    setState(action, element) {
        // const scope = this.findScope(element);
        // scope[action.key] = action.value; // Proxy auto-updates bindings
        console.log("Set state not implemented yet");
    }
}

// Initialize
window.azumi = new Azumi();
console.log("Azumi+ Client Initialized 🚀");
