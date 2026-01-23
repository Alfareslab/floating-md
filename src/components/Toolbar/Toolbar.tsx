import React from 'react';
import { toolbarRegistry, ToolbarItem } from './ToolbarConfig';

interface ToolbarProps {
    orientation: 'horizontal' | 'vertical';
    className?: string;
}

export const Toolbar: React.FC<ToolbarProps> = ({ orientation, className = '' }) => {
    const [items, setItems] = React.useState<ToolbarItem[]>([]);

    React.useEffect(() => {
        setItems(toolbarRegistry.getAll());
    }, []);

    const handleItemClick = async (item: ToolbarItem) => {
        if (item.type === 'separator') return;

        try {
            await item.action();
        } catch (error) {
            console.error(`Error executing ${item.id}:`, error);
        }
    };

    return (
        <div
            className={`
        glass
        flex items-center gap-3 p-3
        ${orientation === 'horizontal' ? 'toolbar-horizontal' : 'toolbar-vertical'}
        ${className}
      `}
            style={{
                borderRadius: '9999px', // Fully rounded capsule
            }}
        >
            {/* Toolbar Items */}
            <div
                className={`
          flex items-center gap-2
          ${orientation === 'horizontal' ? 'flex-row' : 'flex-col'}
        `}
            >
                {items.map((item) => {
                    if (item.type === 'separator') {
                        return (
                            <div
                                key={item.id}
                                className={`
                  bg-white/10
                  ${orientation === 'horizontal' ? 'w-px h-6' : 'h-px w-6'}
                `}
                            />
                        );
                    }

                    return (
                        <button
                            key={item.id}
                            onClick={() => handleItemClick(item)}
                            title={item.tooltip}
                            className={`
                group
                w-10 h-10
                flex items-center justify-center
                rounded-full
                text-text-secondary
                transition-all duration-300
                hover:bg-white/10
                active:scale-95
                hover:text-cyan-400
                hover:shadow-[0_0_15px_rgba(34,211,238,0.4)]
              `}
                        >
                            <span className="transition-transform duration-300 group-hover:scale-110">
                                {item.icon}
                            </span>
                        </button>
                    );
                })}
            </div>
        </div>
    );
};
