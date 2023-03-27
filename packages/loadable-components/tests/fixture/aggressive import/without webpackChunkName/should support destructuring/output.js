loadable({
    resolved: {},
    chunkName ({ foo: foo1  }) {
        return `dir-${foo1}-test`.replace(/[^a-zA-Z0-9_!§$()=\\-^°]+/g, "-");
    },
    isReady (props1) {
        const key1 = this.resolve(props1);
        if (this.resolved[key1] !== true) {
            return false;
        }
        if (typeof __webpack_modules__ !== 'undefined') {
            return !!__webpack_modules__[key1];
        }
        return false;
    },
    importAsync: ({ foo: foo1  })=>import(/*webpackChunkName: "dir-[request]"*/ `./dir/${foo1}/test`),
    requireAsync (props1) {
        const key1 = this.resolve(props1);
        this.resolved[key1] = false;
        return this.importAsync(props1).then((resolved1)=>{
            this.resolved[key1] = true;
            return resolved1;
        });
    },
    requireSync (props) {
        const id = this.resolve(props);
        if (typeof __webpack_require__ !== 'undefined') {
            return __webpack_require__(id);
        }
        return eval('module.require')(id);
    },
    resolve ({ foo  }) {
        if (require.resolveWeak) {
            return require.resolveWeak(`./dir/${foo}/test`);
        }
        return eval('require.resolve')(`./dir/${foo}/test`);
    }
});
