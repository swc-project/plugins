"use client";

const color = "color: red;";

export default function RootLayout({ children }) {
    return (
        <html>
            <head />
            <body>{children}</body>
            <style jsx global>
                {`
          body {
            ${color}
          }
          body p {
            font-size: 72px;
          }
        `}
            </style>
        </html>
    );
}