loadable({
    resolved: {},
    chunkName ({ foo }) {
        return `dir-${foo}-test`.replace(/[^a-zA-Z0-9_!§$()=\\-^°]+/g, "-");
    },
    isReady (props) {
        const key = this.resolve(props);
        if (this.resolved[key] !== true) {
            return false;
        }
        if (typeof __webpack_modules__ !== 'undefined') {
            return !!__webpack_modules__[key];
        }
        return false;
    },
    importAsync: ({ foo })=>import(/*webpackChunkName: "dir-[request]"*/ `./dir/${foo}/test`),
    requireAsync (props) {
        const key = this.resolve(props);
        this.resolved[key] = false;
        return this.importAsync(props).then((resolved)=>{
            this.resolved[key] = true;
            return resolved;
        });
    },
    requireSync (props) {
        const id = this.resolve(props);
        if (typeof __webpack_require__ !== 'undefined') {
            return __webpack_require__(id);
        }
        return eval('module.require')(id);
    },
    resolve ({ foo }) {
        if (require.resolveWeak) {
            return require.resolveWeak(`./dir/${foo}/test`);
        }
        return eval('require.resolve')(`./dir/${foo}/test`);
    }
});
