import loadable from 'my-vendored-loadable';
loadable({
    resolved: {},
    chunkName () {
        return "SomeComponent";
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
    importAsync: ()=>import(/*webpackChunkName: "SomeComponent"*/ './SomeComponent'),
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
    resolve () {
        if (require.resolveWeak) {
            return require.resolveWeak('./SomeComponent');
        }
        return eval('require.resolve')('./SomeComponent');
    }
});
