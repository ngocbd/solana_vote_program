use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vote_program {
        use super::*;

            pub fn add_candidate(ctx: Context<AddCandidate>, nickname: String) -> Result<()> {
                        let candidate = &mut ctx.accounts.candidate;
                                candidate.nickname = nickname;
                                        candidate.votes = 0;
                                                Ok(())
                                                        }

                pub fn vote(ctx: Context<Vote>, candidate_index: u64) -> Result<()> {
                            let candidate = &mut ctx.accounts.candidate;
                                    candidate.votes += 1;
                                            Ok(())
                                                    }

                    pub fn view_vote(ctx: Context<ViewVote>, candidate_index: u64) -> Result<u64> {
                                let candidate = &ctx.accounts.candidate;
                                        Ok(candidate.votes)
                                                }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Candidate {
        pub nickname: String,
            pub votes: u64,
}

#[derive(Accounts)]
pub struct AddCandidate<'info> {
        #[account(init, payer = user, space = 8 + 32 + 8)]
        pub candidate: Account<'info, Candidate>,
            #[account(mut)]
            pub user: Signer<'info>,
                pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
        #[account(mut)]
        pub candidate: Account<'info, Candidate>,
}

#[derive(Accounts)]
pub struct ViewVote<'info> {
        pub candidate: Account<'info, Candidate>,
}
