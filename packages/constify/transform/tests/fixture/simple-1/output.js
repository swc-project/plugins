const __CONST_0__ = {
    code: 1,
    onClick () {}
};
function __CONST_1__() {
    const __data__ = {
        code: 3,
        onClick () {}
    };
    return __CONST_1__ = function() {
        return __data__;
    };
}
;
export function call(dynamic) {
    const options = [
        __CONST_0__,
        {
            code: 2,
            onClick () {
                console.log(dynamic);
            }
        },
        __CONST_1__
    ];
    return options;
}
