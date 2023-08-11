import { constify, lazyConst } from "@swc/constify";

export function call(d1) {
    function a(d2) {
        function a1() {
            return [
                constify({
                    code: d1,
                    onClick() { },
                }),
                constify({
                    code: d2,
                    onClick() {
                        console.log(d2);
                    },
                }),
                lazyConst({
                    code: d1,
                    onClick() { },
                }),
            ];
        }

        return a1;
    }

    return a;
}