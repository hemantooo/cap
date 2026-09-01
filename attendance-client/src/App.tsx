import React from 'react';
import { useDeviceAuth } from './modules/auth/useDeviceAuth';

function App() {
  const { deviceIdentity, loading, error, authenticateDevice } = useDeviceAuth();

  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem' }}>
      <h1>Attendance Client</h1>
      
      <div style={{ background: '#f5f5f5', padding: '1rem', borderRadius: '8px' }}>
        <h2>Device Identity</h2>
        {loading && <p>Loading hardware fingerprint...</p>}
        {error && <p style={{ color: 'red' }}>Error: {error}</p>}
        {deviceIdentity && (
          <div>
            <p><strong>Platform:</strong> {deviceIdentity.platform}</p>
            <p><strong>Fingerprint (SHA-256):</strong> <br /> <code>{deviceIdentity.fingerprint}</code></p>
            <p><strong>Timestamp:</strong> {new Date(deviceIdentity.timestamp).toLocaleString()}</p>
          </div>
        )}
        <button 
          onClick={authenticateDevice} 
          disabled={loading}
          style={{ marginTop: '1rem', padding: '0.5rem 1rem' }}
        >
          Refresh Identity
        </button>
      </div>
    </div>
  );
}

export default App;
