import axios from 'axios';

const api = axios.create({
  baseURL: '/api',
});

export interface Block {
  number: number;
  hash: string;
  timestamp: number;
  transactions: string[];
  validator: string;
}

export interface Transaction {
  hash: string;
  from: string;
  to: string;
  value: string;
  timestamp: number;
}

export interface Validator {
  address: string;
  stake: string;
  isOnline: boolean;
  lastVote: number;
}

export interface NodeMetrics {
  tps: number;
  blockTime: number;
  validatorCount: number;
  networkSize: number;
}

export const getBlocks = async (): Promise<Block[]> => {
  const response = await api.get('/blocks');
  return response.data;
};

export const getTransactions = async (): Promise<Transaction[]> => {
  const response = await api.get('/transactions');
  return response.data;
};

export const getValidators = async (): Promise<Validator[]> => {
  const response = await api.get('/validators');
  return response.data;
};

export const getNodeMetrics = async (): Promise<NodeMetrics> => {
  const response = await api.get('/metrics');
  return response.data;
};
