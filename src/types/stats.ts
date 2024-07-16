export interface Stats {
  nvidia?: StatsNvidia;
  usage: StatsUsage;
}

export interface StatsNvidia {
  temperature: number;
  usage: number;
}

export interface StatsUsage {
  cpu: number;
  memory: number;
}
