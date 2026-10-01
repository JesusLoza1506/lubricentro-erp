import React from 'react';

export const modalInputStyle: React.CSSProperties = {
  width: '100%',
  padding: '10px 12px',
  backgroundColor: 'rgba(30, 41, 59, 0.8)',
  border: '1px solid #334155',
  borderRadius: '8px',
  color: '#F8FAFC',
  fontSize: '13px',
  outline: 'none',
  boxSizing: 'border-box',
};

export const getBadgeStyles = (esCritico: boolean): React.CSSProperties => ({
  padding: '4px 10px',
  borderRadius: '12px',
  fontSize: '12px',
  fontWeight: '700',
  backgroundColor: esCritico ? 'rgba(239, 68, 68, 0.2)' : 'rgba(52, 211, 153, 0.15)',
  color: esCritico ? '#FCA5A5' : '#34D399',
});
