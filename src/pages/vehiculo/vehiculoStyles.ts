import React from 'react';

export const pageContainerStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  minWidth: 0,
  fontFamily: '"Segoe UI Variable", "Segoe UI", sans-serif',
  backgroundColor: '#131D2C',
  backdropFilter: 'none',
  padding: '24px 28px',
  borderRadius: '14px',
  border: '1px solid rgba(51, 65, 85, 0.5)',
  boxShadow: 'none',
  display: 'flex',
  flexDirection: 'column',
  gap: '20px',
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
  borderRadius: '8px',
  padding: '12px 16px 12px 42px',
  color: '#F8FAFC',
  fontSize: '15px',
  fontWeight: '600',
  letterSpacing: '1px',
  outline: 'none',
  width: '230px',
  maxWidth: '100%',
  minHeight: '42px',
  boxSizing: 'border-box',
};
