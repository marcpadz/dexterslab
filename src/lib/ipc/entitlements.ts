import { invokeCommand } from './client';

export async function checkClockRollback(): Promise<boolean> {
  return invokeCommand<boolean>('check_clock_rollback');
}

export async function updateLastExecutionTime(): Promise<void> {
  return invokeCommand<void>('update_last_execution_time');
}

export async function validateLicense(licenseKey: string): Promise<{
  tier: 'free' | 'paid' | 'one_time';
  isLocked: boolean;
  graceExpiresAt: string | null;
}> {
  return invokeCommand('validate_license', { licenseKey });
}
