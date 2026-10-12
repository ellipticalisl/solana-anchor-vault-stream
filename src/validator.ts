import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';

export interface StreamSchedule {
  streamId: BN;
  startTime: BN;
  endTime: BN;
  cliffTime: BN;
  totalDeposit: BN;
}

export class StreamScheduleValidator {
  public static validate(schedule: StreamSchedule): { valid: boolean; error?: string } {
    if (schedule.totalDeposit.lten(0)) {
      return { valid: false, error: 'Total deposit must be greater than zero' };
    }
    if (schedule.endTime.lte(schedule.startTime)) {
      return { valid: false, error: 'End time must be strictly after start time' };
    }
    if (schedule.cliffTime.gt(schedule.endTime)) {
      return { valid: false, error: 'Cliff time cannot exceed stream end time' };
    }
    return { valid: true };
  }

  public static isClaimableAt(schedule: StreamSchedule, timestamp: BN): boolean {
    return timestamp.gte(schedule.cliffTime) && timestamp.gt(schedule.startTime);
  }
}
