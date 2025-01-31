const userIdRegex = /[0-9a-zA-Z_]+/;
export const ATCODER_USER_REGEXP = new RegExp(`^${userIdRegex.source}$`);
export const ATCODER_RIVALS_REGEXP = new RegExp(
  `^\\s*(${userIdRegex.source})\\s*(,\\s*(${userIdRegex.source})\\s*)*$`
);

export const extractRivalsParam = (rivalsParam: string): string[] => {
  if (rivalsParam.match(ATCODER_RIVALS_REGEXP)) {
    return rivalsParam
      .split(",")
      .map(rival => rival.trim())
      .filter(rival => rival.length > 0);
  } else {
    return [];
  }
};

export const isAccepted = (result: string) => result === "AC";
export const ordinalSuffixOf = (i: number) => {
  const j = i % 10;
  const k = i % 100;
  if (j === 1 && k !== 11) {
    return "st";
  }
  if (j === 2 && k !== 12) {
    return "nd";
  }
  if (j === 3 && k !== 13) {
    return "rd";
  }
  return "th";
};

export const clipDifficulty = (difficulty: number) =>
  Math.round(
    difficulty >= 400 ? difficulty : 400 / Math.exp(1.0 - difficulty / 400)
  );
