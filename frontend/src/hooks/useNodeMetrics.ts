import { useQuery } from '@tanstack/react-query';
import { getNodeMetrics } from '@/services/api';

export function useNodeMetrics() {
  return useQuery({
    queryKey: ['nodeMetrics'],
    queryFn: getNodeMetrics,
    refetchInterval: 5000,
  });
}
