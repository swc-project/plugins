import { css } from "@emotion/css";

const wrapFunction = (cb) => {
    return cb();
}

export const classes = wrapFunction(() => {
    const class1 = css({color: "red"});

    return {
        class1,
    }
});