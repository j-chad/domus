export type Colour = 'neutral' | 'primary' | 'secondary' | 'accent' | 'ghost' | 'link';

export const ButtonColours: Record<Colour, string> = {
	neutral: 'btn-neutral',
	primary: 'btn-primary',
	secondary: 'btn-secondary',
	accent: 'btn-accent',
	ghost: 'btn-ghost',
	link: 'btn-link'
};
