import React from 'react';

export const pageContainerStyle: React.CSSProperties = {
  flex: 1,
  backgroundColor: 'rgba(30, 41, 59, 0.4)',
  backdropFilter: 'blur(16px)',
  padding: '36px',
  borderRadius: '20px',
  border: '1px solid rgba(51, 65, 85, 0.5)',
  boxShadow: '0 20px 40px rgba(0, 0, 0, 0.4)',
  display: 'flex',
  flexDirection: 'column',
  gap: '28px',
  overflowY: 'auto',
  position: 'relative',
};

export const btnPlacaPruebaStyle: React.CSSProperties = {
  backgroundColor: '#1E293B',
  border: '1px solid #38BDF8',
  color: '#38BDF8',
  padding: '2px 8px',
  borderRadius: '6px',
  cursor: 'pointer',
  fontSize: '12px',
  fontWeight: '700',
};

export const inputBusquedaStyle: React.CSSProperties = {
  backgroundColor: 'rgba(15, 23, 42, 0.8)',
  border: '1px solid #334155',
  borderRadius: '12px',
  padding: '12px 16px 12px 42px',
  color: '#F8FAFC',
  fontSize: '15px',
  fontWeight: '600',
  letterSpacing: '1px',
  outline: 'none',
  width: '180px',
};
