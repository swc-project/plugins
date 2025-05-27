export default function IndexPage() {
    return (
        <div>
            <h1>Hello World.</h1>

            <style jsx global>{`
                .container {
                    & > div {
                        color: green;
                    }
                }
        `}</style>
        </div>
    );
}
