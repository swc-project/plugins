export const Probe = (props) => {
  const t = props.theme;
  const Icon = t.icon;
  return (
    <div className="probe">
      <Icon />
      <style jsx>{`
        .probe {
          color: ${t.color};
        }
      `}</style>
    </div>
  );
};
