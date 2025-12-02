class Azumi {
    constructor() {
        this.scopes = new WeakMap(); // Element -> Proxy
        this.delegate();
        this.initScopes();
    }

    // Initialize all az-scope elements
    initScopes() {
        document.querySelectorAll("[az-scope]").forEach((el) => {
            try {
                const data = JSON.parse(el.getAttribute("az-scope"));
                this.createScope(el, data);
            } catch (e) {
                console.error("Failed to parse az-scope:", e);
            }
        });
    }

    // Create a reactive scope
    createScope(element, data) {
        const self = this;
        const proxy = new Proxy(data, {
            set(target, key, value) {
                target[key] = value;
                self.updateBindings(element, key, value);
                return true;
            },
        });
        this.scopes.set(element, proxy);
        // Initial render of bindings
        this.renderBindings(element, proxy);
    }

    // Find the scope for an element (walk up the tree)
    findScope(element) {
        let current = element;
        while (current) {
            if (this.scopes.has(current)) {
                return this.scopes.get(current);
            }
            current = current.parentElement;
        }
        return null;
    }

    // Render all bindings within a scope
    renderBindings(scopeEl, scope) {
        // az-bind:text="key"
        scopeEl.querySelectorAll("[az-bind\\:text]").forEach((el) => {
            const expr = el.getAttribute("az-bind:text");
            const value = this.evalExpr(expr, scope);
            if (value !== undefined) el.textContent = value;
        });

        // az-bind:class.foo="condition"
        for (const attr of scopeEl.attributes || []) {
            if (attr.name.startsWith("az-bind:class.")) {
                const className = attr.name.slice("az-bind:class.".length);
                const condition = attr.value;
                const shouldHave = this.evalExpr(condition, scope);
                scopeEl.classList.toggle(className, !!shouldHave);
            }
        }
    }

    // Update bindings when state changes
    updateBindings(scopeEl, key, value) {
        this.renderBindings(scopeEl, this.scopes.get(scopeEl));
    }

    // Simple expression evaluation
    evalExpr(expr, scope) {
        // Handle simple cases: "count", "liked", "count + 1", etc.
        try {
            const func = new Function(...Object.keys(scope), `return ${expr}`);
            return func(...Object.values(scope));
        } catch (e) {
            console.warn("Failed to eval expression:", expr, e);
            return undefined;
        }
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

        const attr = target.getAttribute("az-on");
        const parts = attr.split(" ");
        const trigger = parts[0];

        if (trigger !== e.type) return;

        e.preventDefault();

        const action = this.parseAction(parts.slice(1).join(" "), target);
        if (action) this.execute(action, target);
    }

    parseAction(cmd, element) {
        const tokens = cmd.split(" ");
        const type = tokens[0]; // "call" or "set"

        if (type === "call") {
            let url = tokens[1];
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

        if (type === "set") {
            // Format: "set count = count + 1"
            const rest = tokens.slice(1).join(" ");
            const eqIndex = rest.indexOf("=");
            if (eqIndex === -1) return null;

            const key = rest.slice(0, eqIndex).trim();
            const valueExpr = rest.slice(eqIndex + 1).trim();

            return { type: "set", key, valueExpr };
        }

        return null;
    }

    // Execute: "call toggle_like -> #box" or "set count = count + 1"
    async execute(action, element) {
        if (action.type === "call") {
            await this.callAction(action, element);
        } else if (action.type === "set") {
            this.setState(action, element);
        }
    }

    // Server action
    async callAction(action, element) {
        // Get scope data to send to server
        const scopeEl = element.closest("[az-scope]");
        let body = {};

        if (scopeEl && this.scopes.has(scopeEl)) {
            const scope = this.scopes.get(scopeEl);
            body = { ...scope };
        }

        try {
            const res = await fetch(action.url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(body),
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
                target.outerHTML = html;
            }

            // Re-initialize scopes after DOM update
            setTimeout(() => this.initScopes(), 10);
        } catch (err) {
            console.error("Azumi action error:", err);
        }
    }

    // Local state change
    setState(action, element) {
        const scopeEl = element.closest("[az-scope]");
        if (!scopeEl) {
            console.warn("No az-scope found for setState");
            return;
        }

        const scope = this.scopes.get(scopeEl);
        if (!scope) {
            console.warn("Scope not initialized");
            return;
        }

        // Evaluate the expression
        const value = this.evalExpr(action.valueExpr, scope);
        scope[action.key] = value; // This triggers the proxy setter
    }
}

// Initialize
window.azumi = new Azumi();
console.log("Azumi+ Client Initialized 🚀");
