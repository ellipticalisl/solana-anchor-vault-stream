import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';

export interface StreamParams {
  streamId: BN;
  totalAmount: BN;
  startTime: BN;
  endTime: BN;
  cliffTime: BN;
}

export class SolanaStreamClient {
  public static readonly PROGRAM_ID = new PublicKey('StrmV11111111111111111111111111111111111111');

  public static findStreamAddress(sender: PublicKey, streamId: BN): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from('stream'),
        sender.toBuffer(),
        streamId.toArrayLike(Buffer, 'le', 8),
      ],
      this.PROGRAM_ID
    );
  }

  public static findVaultAddress(streamAccount: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), streamAccount.toBuffer()],
      this.PROGRAM_ID
    );
  }

  public static computeVestedAmount(
    totalAmount: BN,
    startTime: BN,
    endTime: BN,
    currentTime: BN
  ): BN {
    if (currentTime.lte(startTime)) {
      return new BN(0);
    }
    if (currentTime.gte(endTime)) {
      return totalAmount;
    }
    const elapsed = currentTime.sub(startTime);
    const duration = endTime.sub(startTime);
    return totalAmount.mul(elapsed).div(duration);
  }
}
