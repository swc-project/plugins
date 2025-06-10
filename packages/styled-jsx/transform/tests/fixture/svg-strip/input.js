const CUTOUT_AVATAR_PERCENTAGE_VISIBLE = Math.random();
const HEAD_MARGIN_PERCENTAGE = Math.random();

const MaskedDivBad = () => {


    return (
        <>
            <div className="head">
                <div className="avatar-wrapper" />
            </div>
            <style jsx>{`
          .head {
            position: relative;
          }
          .avatar-wrapper {
            width: 40px;
            height: 40px;
            background: #ff6b6b;
            border-radius: 50%;
            mask-image:
              url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"><circle r="0.5" cx="0.5" cy="0.5"/></svg>'),
              url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"><circle r="${0.5 +
                HEAD_MARGIN_PERCENTAGE}" cx="${0.5 +
                CUTOUT_AVATAR_PERCENTAGE_VISIBLE +
                HEAD_MARGIN_PERCENTAGE}" cy="0.5"/></svg>');
            mask-size: 100% 100%;
            mask-repeat: no-repeat;
            mask-position: center;
            -webkit-mask-composite: source-out;
            mask-composite: subtract;
          }
        `}</style>
        </>
    );
};