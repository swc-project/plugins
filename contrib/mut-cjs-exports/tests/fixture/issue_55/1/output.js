Object.keys(mod).forEach(function(key) {
    if (key === "default" || key === "__esModule") return;
    if (Object.prototype.hasOwnProperty.call(exports, key)) return;
    Object.defineProperty(exports, key, {
        enumerable: true,
        get: function() {
            return mod[key];
        },
        configurable: true
    });
});
import * as mod from "./someModule";
