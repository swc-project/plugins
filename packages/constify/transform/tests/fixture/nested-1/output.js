;
export function call(d1) {
    const __CONST_0__ = {
        code: d1,
        onClick () {}
    };
    function __CONST_2__() {
        const __data__ = {
            code: d1,
            onClick () {}
        };
        return __CONST_2__ = function() {
            return __data__;
        };
    }
    function a(d2) {
        const __CONST_1__ = {
            code: d2,
            onClick () {
                console.log(d2);
            }
        };
        function a1() {
            return [
                __CONST_0__,
                __CONST_1__,
                __CONST_2__
            ];
        }
        return a1;
    }
    return a;
}
