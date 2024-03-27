/* eslint-disable-next-line import/no-default-export -- TODO: Fix ESLint Error (#13355) */
export default function ReactCropStyle() {
  return (
    <style global jsx>{`
      .ReactCrop__crop-selection {
        border-image-source: url("https://example.com/image.png"); //
      }
    `}</style>
  );
}
