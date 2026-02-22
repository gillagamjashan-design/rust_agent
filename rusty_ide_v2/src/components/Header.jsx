import React, { useState } from 'react';

const Header = ({
  onToggleFileTree,
  onToggleTerminal,
  onToggleAgent,
  onNewFile,
  onOpenFile,
  onSaveFile
}) => {
  const [openMenu, setOpenMenu] = useState(null);

  const menuItems = {
    File: [
      { label: 'New File', shortcut: 'Ctrl+N', action: onNewFile },
      { label: 'Open File', shortcut: 'Ctrl+O', action: onOpenFile },
      { label: 'Save', shortcut: 'Ctrl+S', action: onSaveFile },
      { type: 'separator' },
      { label: 'Exit', shortcut: 'Ctrl+Q', action: () => window.close() }
    ],
    Edit: [
      { label: 'Undo', shortcut: 'Ctrl+Z' },
      { label: 'Redo', shortcut: 'Ctrl+Y' },
      { type: 'separator' },
      { label: 'Cut', shortcut: 'Ctrl+X' },
      { label: 'Copy', shortcut: 'Ctrl+C' },
      { label: 'Paste', shortcut: 'Ctrl+V' }
    ],
    View: [
      { label: 'Toggle File Explorer', action: onToggleFileTree },
      { label: 'Toggle Terminal', action: onToggleTerminal },
      { label: 'Toggle AI Sidebar', action: onToggleAgent }
    ],
    Terminal: [
      { label: 'New Terminal', shortcut: 'Ctrl+`' },
      { label: 'Split Terminal' },
      { label: 'Kill Terminal' }
    ]
  };

  const handleMenuClick = (menuName) => {
    setOpenMenu(openMenu === menuName ? null : menuName);
  };

  const handleMenuItemClick = (item) => {
    if (item.action) {
      item.action();
    }
    setOpenMenu(null);
  };

  return (
    <div className="header">
      <div className="header-menu">
        {Object.keys(menuItems).map((menuName) => (
          <div key={menuName} style={{ position: 'relative' }}>
            <div
              className="header-menu-item"
              onClick={() => handleMenuClick(menuName)}
            >
              {menuName}
            </div>
            {openMenu === menuName && (
              <div
                style={{
                  position: 'absolute',
                  top: '100%',
                  left: 0,
                  background: 'var(--bg-elevated)',
                  border: '1px solid var(--border-primary)',
                  borderRadius: 'var(--radius-md)',
                  minWidth: '200px',
                  padding: 'var(--spacing-xs)',
                  zIndex: 1000,
                  boxShadow: 'var(--shadow-lg)'
                }}
                onMouseLeave={() => setOpenMenu(null)}
              >
                {menuItems[menuName].map((item, index) => (
                  item.type === 'separator' ? (
                    <div
                      key={index}
                      style={{
                        height: '1px',
                        background: 'var(--border-primary)',
                        margin: 'var(--spacing-xs) 0'
                      }}
                    />
                  ) : (
                    <div
                      key={index}
                      onClick={() => handleMenuItemClick(item)}
                      style={{
                        padding: 'var(--spacing-xs) var(--spacing-md)',
                        cursor: 'pointer',
                        borderRadius: 'var(--radius-sm)',
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        fontSize: '13px'
                      }}
                      onMouseEnter={(e) => {
                        e.currentTarget.style.background = 'var(--accent-primary)';
                      }}
                      onMouseLeave={(e) => {
                        e.currentTarget.style.background = 'transparent';
                      }}
                    >
                      <span>{item.label}</span>
                      {item.shortcut && (
                        <span style={{ color: 'var(--text-muted)', fontSize: '11px' }}>
                          {item.shortcut}
                        </span>
                      )}
                    </div>
                  )
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
      <div className="header-actions">
        <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>
          Rusty IDE v2
        </div>
      </div>
    </div>
  );
};

export default Header;
