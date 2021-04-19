export const truncate = str => {
  const [head, tail] = str.split(/(.{8}).*(.{8})/).filter(Boolean);
  return `${head}…${tail}`;
};
