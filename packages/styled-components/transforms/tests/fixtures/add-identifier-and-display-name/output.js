import styled from "styled-components";
const Test = styled.div.withConfig({
    displayName: "Test",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-0"
})`
  width: 100%;
`;
const Test2 = true ? styled.div.withConfig({
    displayName: "Test2",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-1"
})`` : styled.div.withConfig({
    displayName: "Test2",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-2"
})``;
const styles = {
    One: styled.div.withConfig({
        displayName: "One",
        componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-3"
    })``
};
let Component;
Component = styled.div.withConfig({
    displayName: "Component",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-4"
})``;
const WrappedComponent = styled(Inner).withConfig({
    displayName: "WrappedComponent",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-5"
})``;
const WrappedComponent2 = styled.div.withConfig({
    displayName: "WrappedComponent2",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-6"
})({
});
const WrappedComponent3 = styled(Inner).withConfig({
    displayName: "WrappedComponent3",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-7"
})({
});
const WrappedComponent4 = styled(Inner).attrs(()=>({
        something: "else"
    })
)({
});
const WrappedComponent5 = styled.div.attrs(()=>({
        something: "else"
    })
)({
});
const WrappedComponent6 = styled.div.attrs(()=>({
        something: "else"
    })
).withConfig({
    displayName: "WrappedComponent6",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-8"
})``;
const WrappedComponent7 = styled.div.withConfig({
    shouldForwardProp: ()=>{
    },
    displayName: "WrappedComponent7",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-9"
})({
});
const WrappedComponent8 = styled.div.withConfig({
    shouldForwardProp: ()=>{
    }
}).attrs(()=>({
        something: "else"
    })
)({
});
const WrappedComponent9 = styled.div.attrs(()=>({
        something: "else"
    })
).withConfig({
    shouldForwardProp: ()=>{
    },
    displayName: "WrappedComponent9",
    componentId: "sc-3712c9905a45f85c54cdd4f4d94f5295-10"
})({
});
