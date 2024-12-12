use anchor_lang::prelude::*;  // 引入 Anchor 预设模块 
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer};  // 引入 SPL Token 相关模块

declare_id!("DjDunrSjm91ZHcFQT1Ga7Y8DoV5GnnecdbVoXEtLwXZv");  // 设置合约 ID

// 固定的元数据 URI
const METADATA_URI: &str = "https://aquamarine-additional-carp-588.mypinata.cloud/ipfs/bafkreiaax3gsuvfttrixq3iw7ki37ljqssnfk655dts5rnkpdrowug4ygy";

// 自定义账户，用于存储 URI 信息
#[account]
pub struct CustomMetadata {
    pub uri: String,  // URI 字符串
}

#[program]
pub mod usdt_metadata {
    use super::*;

    // 设置代币的自定义元数据（绕过审核）
    pub fn set_metadata(ctx: Context<SetMetadata>) -> Result<()> {
        let metadata_account = &mut ctx.accounts.metadata;
        metadata_account.uri = METADATA_URI.to_string(); // 使用固定的 URI
        Ok(())
    }

    // 获取代币的自定义元数据（输出到日志）
    pub fn get_metadata(ctx: Context<GetMetadata>) -> Result<()> {
        let metadata_account = &ctx.accounts.metadata;
        msg!("Metadata URI: {}", metadata_account.uri); // 输出元数据 URI
        Ok(())
    }

    // 铸造代币（限制到指定账户）
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        let mint_account = &ctx.accounts.mint;
        let authority = &ctx.accounts.authority;

        // 固定目标账户地址（指定账户：F5BVciapAXfL6YiiFgDtLNDwL96drBVZCqrKnkzpWd5V）
        let target_account: Pubkey = "F5BVciapAXfL6YiiFgDtLNDwL96drBVZCqrKnkzpWd5V".parse().unwrap();

        // 检查权限，确保当前调用者是 `authority`
        if *authority.key != target_account {
            return Err(ProgramError::IllegalOwner.into()); // 非授权账户不能铸币
        }

        // 使用 SPL Token 程序铸造代币到目标账户
        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: mint_account.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    // 限制只有指定用户能兑换代币
    pub fn exchange_token(ctx: Context<ExchangeToken>, amount: u64) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let target_account: Pubkey = "F5BVciapAXfL6YiiFgDtLNDwL96drBVZCqrKnkzpWd5V".parse().unwrap();

        // 检查权限，确保只有 `authority` 账户能够进行兑换操作
        if *authority.key != target_account {
            return Err(ProgramError::IllegalOwner.into()); // 非授权账户不能兑换
        }

        // 执行兑换操作（这里只是一个示范，实际逻辑可能需要处理目标代币等）
        msg!("Exchange operation for {} tokens.", amount);
        Ok(())
    }

    // 允许用户在钱包之间转账代币
    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        let from_token_account = &ctx.accounts.from_token_account;
        let to_token_account = &ctx.accounts.to_token_account;

        // 使用 SPL Token 程序进行转账
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: from_token_account.to_account_info(),
                    to: to_token_account.to_account_info(),
                    authority: from_token_account.to_account_info(), 
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

// 上下文结构体

// 用于设置元数据的上下文
#[derive(Accounts)]
pub struct SetMetadata<'info> {
    #[account(mut)]
    pub metadata: Account<'info, CustomMetadata>, // 自定义元数据账户
    pub payer: Signer<'info>,
}

// 用于获取元数据的上下文
#[derive(Accounts)]
pub struct GetMetadata<'info> {
    pub metadata: Account<'info, CustomMetadata>, // 自定义元数据账户
}

// 用于铸造代币的上下文
#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,  // 代币铸造账户
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, // 目标账户
    pub authority: Signer<'info>,  // 授权用户
    pub token_program: Program<'info, Token>, // Token 程序
}

// 用于兑换代币的上下文
#[derive(Accounts)]
pub struct ExchangeToken<'info> {
    pub authority: Signer<'info>,  // 授权账户
}

// 用于转账代币的上下文
#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>, // 发送方代币账户
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,   // 接收方代币账户
    pub token_program: Program<'info, Token>,              // SPL Token 程序
}


