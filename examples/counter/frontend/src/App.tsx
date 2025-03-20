import { useState, useEffect } from 'react';
import { useRustorium, useContract } from '@rustorium/react';
import { Counter } from '../../contracts/counter';
import './App.css';

function App() {
  const { isConnected, connect } = useRustorium();
  const [count, setCount] = useState<number>(0);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const counter = useContract<Counter>(import.meta.env.VITE_COUNTER_ADDRESS);

  useEffect(() => {
    if (counter) {
      updateCount();
      subscribeToEvents();
    }
  }, [counter]);

  async function updateCount() {
    try {
      const value = await counter.get_count();
      setCount(value);
      setError(null);
    } catch (err) {
      setError('Failed to get count');
      console.error(err);
    }
  }

  async function handleIncrement() {
    if (loading) return;
    setLoading(true);
    try {
      await counter.increment();
      await updateCount();
      setError(null);
    } catch (err) {
      setError('Failed to increment');
      console.error(err);
    } finally {
      setLoading(false);
    }
  }

  async function handleDecrement() {
    if (loading) return;
    setLoading(true);
    try {
      await counter.decrement();
      await updateCount();
      setError(null);
    } catch (err) {
      setError('Failed to decrement');
      console.error(err);
    } finally {
      setLoading(false);
    }
  }

  function subscribeToEvents() {
    const subscription = counter.events.CounterChanged.subscribe(
      (event) => {
        console.log('Counter changed:', {
          oldValue: event.old_value,
          newValue: event.new_value,
          changedBy: event.changed_by,
        });
        setCount(event.new_value);
      },
      (err) => {
        console.error('Event subscription error:', err);
        setError('Failed to subscribe to events');
      }
    );

    return () => subscription.unsubscribe();
  }

  if (!isConnected) {
    return (
      <div className="container">
        <button className="connect-button" onClick={connect}>
          Connect to Rustorium
        </button>
      </div>
    );
  }

  return (
    <div className="container">
      <h1>Counter App</h1>
      
      <div className="counter">
        <h2>Count: {count}</h2>
        
        <div className="buttons">
          <button
            className="button decrement"
            onClick={handleDecrement}
            disabled={loading}
          >
            -
          </button>
          
          <button
            className="button increment"
            onClick={handleIncrement}
            disabled={loading}
          >
            +
          </button>
        </div>
        
        {loading && (
          <div className="loading">
            Processing...
          </div>
        )}
        
        {error && (
          <div className="error">
            {error}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
