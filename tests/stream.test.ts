import { describe, it, expect } from 'vitest';
import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';
import { SolanaStreamClient } from '../src/client.js';

describe('SolanaStreamClient', () => {
  const dummySender = new PublicKey('11111111111111111111111111111111');

  it('derives deterministic stream PDA and bump', () => {
    const streamId = new BN(1);
    const [pda1, bump1] = SolanaStreamClient.findStreamAddress(dummySender, streamId);
    const [pda2, bump2] = SolanaStreamClient.findStreamAddress(dummySender, streamId);

    expect(pda1.toBase58()).toBe(pda2.toBase58());
    expect(bump1).toBe(bump2);
  });

  it('calculates linear vesting amount accurately', () => {
    const totalAmount = new BN(1_000_000);
    const startTime = new BN(1000);
    const endTime = new BN(2000);

    // Before start
    expect(SolanaStreamClient.computeVestedAmount(totalAmount, startTime, endTime, new BN(900)).toNumber()).toBe(0);

    // Exactly half-way
    expect(SolanaStreamClient.computeVestedAmount(totalAmount, startTime, endTime, new BN(1500)).toNumber()).toBe(500_000);

    // After finish
    expect(SolanaStreamClient.computeVestedAmount(totalAmount, startTime, endTime, new BN(2500)).toNumber()).toBe(1_000_000);
  });
});
