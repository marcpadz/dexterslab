class AuthStore {
  tier = $state<'free' | 'paid' | 'one_time'>('free');
  isLocked = $state(false);
  graceExpiresAt = $state<Date | null>(null);
  quotas = $state({
    dailyMessages: 10,
    kbFiles: 2
  });

  updateStatus(status: {
    tier: 'free' | 'paid' | 'one_time';
    isLocked: boolean;
    graceExpiresAt: string | null;
  }) {
    this.tier = status.tier;
    this.isLocked = status.isLocked;
    this.graceExpiresAt = status.graceExpiresAt ? new Date(status.graceExpiresAt) : null;
    
    // Set quotas based on tier
    if (this.tier === 'paid' || this.tier === 'one_time') {
      this.quotas = { dailyMessages: -1, kbFiles: -1 }; // unlimited
    } else {
      this.quotas = { dailyMessages: 10, kbFiles: 2 };
    }
  }
}

export const authStore = new AuthStore();
