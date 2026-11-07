import React from 'react';
import logo from '../../../assets/logo-gloria-login.png';

interface LoginBrandPanelProps {
  appVersion: string;
}

export const LoginBrandPanel: React.FC<LoginBrandPanelProps> = ({ appVersion }) => (
  <section className="gloria-brand" aria-label="Lubricentro Gloria">
    <header className="gloria-brand-header">
      <img className="gloria-brand-logo" src={logo} alt="Lubricentro Gloria" />
      <span className="gloria-brand-caption">Sistema de gestión</span>
    </header>

    <div className="gloria-brand-copy">
      <h1>El taller, en orden.</h1>
      <p>Vehículos, servicios, inventario y ventas en un solo lugar.</p>
    </div>

    <footer className="gloria-brand-footer">
      <span>© 2026 Lubricentro Gloria S.R.L.</span>
      <span>Versión {appVersion}</span>
    </footer>
  </section>
);
