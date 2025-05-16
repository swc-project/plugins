export default function Home() {
    return (
        <>
            <div className="container">
                container (should be blue)

                <div className="child">
                    this should be green
                </div>
            </div>


            <style jsx>{`
            .container {
                color: blue;
                padding: 3rem;
            }
  
            :global(div.child) {
                color: green;
            }
        `}</style>
        </>
    );
}