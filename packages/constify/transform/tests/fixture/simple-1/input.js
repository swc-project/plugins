import { constify, lazyConst } from "@swc/constify";

export function call(dynamic) {
    const options = [
        constify({
            code: 1,
            onClick() { },
        }),
        {
            code: 2,
            onClick() {
                console.log(dynamic);
            },
        },
        lazyConst({
            code: 3,
            onClick() { },
        }),
    ];

    return options;
}