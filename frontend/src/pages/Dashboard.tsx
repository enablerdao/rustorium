import {
  Box,
  SimpleGrid,
  Stat,
  StatLabel,
  StatNumber,
  StatHelpText,
  Card,
  CardBody,
  Heading,
} from '@chakra-ui/react';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { useNodeMetrics } from '@/hooks/useNodeMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export default function Dashboard() {
  const { data: metrics, isLoading } = useNodeMetrics();

  const tpsChartData = {
    labels: ['1m', '2m', '3m', '4m', '5m'],
    datasets: [
      {
        label: 'TPS',
        data: [65, 59, 80, 81, 56],
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1,
      },
    ],
  };

  return (
    <Box>
      <Heading mb={6}>Dashboard</Heading>
      <SimpleGrid columns={{ base: 1, md: 2, lg: 4 }} spacing={4} mb={8}>
        <Card>
          <CardBody>
            <Stat>
              <StatLabel>TPS</StatLabel>
              <StatNumber>1,234</StatNumber>
              <StatHelpText>↑ 23%</StatHelpText>
            </Stat>
          </CardBody>
        </Card>
        <Card>
          <CardBody>
            <Stat>
              <StatLabel>Block Time</StatLabel>
              <StatNumber>1.2s</StatNumber>
              <StatHelpText>↓ 5%</StatHelpText>
            </Stat>
          </CardBody>
        </Card>
        <Card>
          <CardBody>
            <Stat>
              <StatLabel>Validators</StatLabel>
              <StatNumber>21</StatNumber>
              <StatHelpText>Active</StatHelpText>
            </Stat>
          </CardBody>
        </Card>
        <Card>
          <CardBody>
            <Stat>
              <StatLabel>Network Size</StatLabel>
              <StatNumber>1.2 TB</StatNumber>
              <StatHelpText>↑ 10%</StatHelpText>
            </Stat>
          </CardBody>
        </Card>
      </SimpleGrid>

      <SimpleGrid columns={{ base: 1, lg: 2 }} spacing={4}>
        <Card>
          <CardBody>
            <Heading size="md" mb={4}>
              Transactions Per Second
            </Heading>
            <Box h="300px">
              <Line
                data={tpsChartData}
                options={{
                  responsive: true,
                  maintainAspectRatio: false,
                }}
              />
            </Box>
          </CardBody>
        </Card>
        <Card>
          <CardBody>
            <Heading size="md" mb={4}>
              Block Time
            </Heading>
            <Box h="300px">
              <Line
                data={tpsChartData}
                options={{
                  responsive: true,
                  maintainAspectRatio: false,
                }}
              />
            </Box>
          </CardBody>
        </Card>
      </SimpleGrid>
    </Box>
  );
}
