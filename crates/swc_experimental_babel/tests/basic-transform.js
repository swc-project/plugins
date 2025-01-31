

function transform({ code, map }) {


    return {
        code: code + ';\nconsole.log("hello")',
        map,
    };
}
