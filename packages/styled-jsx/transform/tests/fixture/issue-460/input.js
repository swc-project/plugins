export default function Home() {
    return (
        <>
            <style jsx>{`
            div {
                mask-image: -webkit-radial-gradient(
                    white,
                    black
                ); // Fixing a Safari bug where the corners of the polaroid are not rounded properly
            }
        `}</style>
        </>
    );
}